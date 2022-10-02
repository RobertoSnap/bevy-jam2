use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::{
    action_state::ActionDiff, prelude::InputManagerPlugin,
    systems::process_action_diffs,
};
use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::{
    renet::{
        RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
        ServerEvent,
    },
    RenetServerPlugin,
};

use crate::{
    input::Action,
    movement::Movement,
    network::shared::ServerMessages,
    player::{Player, PlayerBundle},
};

use crate::network::shared::Lobby;

use super::shared::NetworkID;
pub struct ServerPlugin;

const PROTOCOL_ID: u64 = 7;

// let mut server = new_renet_server();

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin)
            .insert_resource(new_renet_server())
            .add_system(send_message_system)
            .add_system(sync_input)
            .add_system(update_system)
            .add_plugin(InputManagerPlugin::<Action>::server())
            .add_event::<ActionDiff<Action, NetworkID>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                process_action_diffs::<Action, NetworkID>,
            )
            .insert_resource(Lobby::default());
    }
}

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // server.broadcast_message(channel_id, "server message".as_bytes().to_vec());
}
fn sync_input(
    mut server: ResMut<RenetServer>,
    mut events: EventWriter<ActionDiff<Action, NetworkID>>,
) {
    let channel_id = 1;
    // Send a text message for all clients
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, channel_id)
        {
            // TODO : Receive all event batched
            let input: ActionDiff<Action, NetworkID> =
                bincode::deserialize(&message).unwrap();
            events.send(input.clone());
            // Handle received message
        }
    }
}

fn update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main server events
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Client {} connected", id);
                // spawn player on server
                let mesh_handle =
                    meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
                let material_handle =
                    materials.add(Color::rgb(0.8, 0.7, 0.6).into());
                let player_id = commands
                    .spawn_bundle(PlayerBundle::new(
                        *id,
                        mesh_handle,
                        material_handle,
                    ))
                    .id();
                // init players on client
                for &player_id in lobby.players.keys() {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected {
                            id: player_id,
                        })
                        .unwrap();
                    server.send_message(*id, 0, message);
                }

                // Insert player network to entity mapping on server
                lobby.players.insert(*id, player_id);

                // Init player on other clients
                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected {
                        id: *id,
                    })
                    .unwrap();
                server.broadcast_message(0, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Client {} disconnected", id);
                if let Some(player_entity) = lobby.players.remove(id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected {
                        id: *id,
                    })
                    .unwrap();
                server.broadcast_message(0, message);
            }
        }
    }

    // client events
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, 0) {
            // let player_input: PlayerInput =
            //     bincode::deserialize(&message).unwrap();
            // if let Some(player_entity) = lobby.players.get(&client_id) {
            //     commands.entity(*player_entity).insert(player_input);
            // }
        }
    }
}

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(
        64,
        PROTOCOL_ID,
        server_addr,
        ServerAuthentication::Unsecure,
    );
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket)
        .unwrap()
}
