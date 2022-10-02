use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy_rapier3d::prelude::*;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use leafwing_input_manager::{
    action_state::ActionDiff,
    prelude::{ActionState, InputManagerPlugin, InputMap},
    systems::generate_action_diffs,
    InputManagerBundle,
};

use super::shared::{Lobby, NetworkID, ServerMessages};

use crate::{
    input::Action,
    movement::Movement,
    player::{Player, PlayerBundle, PlayerCam},
};
pub struct ClientPlugin;

pub struct MyNetworkID(u64);

const PROTOCOL_ID: u64 = 7;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            .insert_resource(new_renet_client())
            .insert_resource(Lobby::default())
            .insert_resource(NetworkID::default())
            .insert_resource(MyNetworkID(0))
            .add_system(
                client_sync_players.with_run_criteria(run_if_client_connected),
            )
            .add_plugin(InputManagerPlugin::<Action>::default())
            // Creates an event stream of `ActionDiffs` to send to the server
            .add_system_to_stage(
                CoreStage::PostUpdate,
                generate_action_diffs::<Action, NetworkID>,
            )
            .add_event::<ActionDiff<Action, NetworkID>>()
            .add_system(sync_inputs.with_run_criteria(run_if_client_connected));
    }
}

fn sync_inputs(
    mut events: EventReader<ActionDiff<Action, NetworkID>>,
    mut client: ResMut<RenetClient>,
) {
    // TODO : Send all event batched
    for mut event in events.iter() {
        let message = bincode::serialize(event).unwrap();
        client.send_message(1, message);
    }
}

fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    while let Some(message) = client.receive_message(0) {
        use Action::*;
        use KeyCode::*;
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                let mesh_handle =
                    meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
                let material_handle =
                    materials.add(Color::rgb(0.8, 0.7, 0.6).into());
                let player_id = commands
                    .spawn_bundle(PlayerBundle::new(
                        id,
                        mesh_handle,
                        material_handle,
                    ))
                    .id();

                if client.client_id() == id {
                    commands
                        .entity(player_id)
                        .insert_bundle(InputManagerBundle {
                            input_map: InputMap::new([
                                (A, MoveLeft),
                                (D, MoveRight),
                                (W, MoveUp),
                                (S, MoveDown),
                                (Space, Jump),
                            ])
                            .insert(MouseButton::Left, Shoot)
                            .build(),
                            action_state: ActionState::default(),
                        })
                        .with_children(|commands| {
                            commands
                                .spawn_bundle(Camera3dBundle {
                                    transform: Transform::from_xyz(
                                        0.0, 0.5, 0.0,
                                    ),
                                    projection: Projection::Perspective(
                                        PerspectiveProjection {
                                            fov: 90.0
                                                * (std::f32::consts::PI
                                                    / 180.0),
                                            aspect_ratio: 1.0,
                                            near: 0.3,
                                            far: 1000.0,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(PlayerCam)
                                .with_children(|commands| {
                                    let mesh = meshes
                                        .add(shape::Cube { size: 0.5 }.into());

                                    commands.spawn_bundle(PbrBundle {
                                        mesh,
                                        material: materials
                                            .add(Color::WHITE.into()),
                                        transform: Transform::from_xyz(
                                            0.0, 0.0, -0.5,
                                        ),
                                        ..default()
                                    });
                                });
                        });
                }

                lobby.players.insert(id, player_id);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
            ServerMessages::PlayerJump { id } => {
                println!("SOmeone jumping");
            }
        }
    }

    while let Some(message) = client.receive_message(1) {
        let players: HashMap<u64, [f32; 3]> =
            bincode::deserialize(&message).unwrap();
        for (player_id, translation) in players.iter() {
            if let Some(player_entity) = lobby.players.get(player_id) {
                let transform = Transform {
                    translation: (*translation).into(),
                    ..Default::default()
                };
                commands.entity(*player_entity).insert(transform);
            }
        }
    }
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(
        current_time,
        socket,
        client_id,
        connection_config,
        authentication,
    )
    .unwrap()
}
