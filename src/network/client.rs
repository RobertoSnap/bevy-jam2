use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};

use crate::player::Player;

use super::shared::{Lobby, ServerMessages};

pub struct ClientPlugin;

const PROTOCOL_ID: u64 = 7;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            .insert_resource(new_renet_client())
            .insert_resource(Lobby::default())
            .add_system(
                client_sync_players.with_run_criteria(run_if_client_connected),
            );
    }
}

fn client_sync_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                let player_entity = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            ..Default::default()
                        },
                        transform: Transform {
                            scale: Vec3::new(16.0, 16.0, 1.0),
                            translation: Vec3::new(0.0, 0.0, 10.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player { id: id })
                    .id();

                lobby.players.insert(id, player_entity);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
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
