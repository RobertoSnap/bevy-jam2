use bevy::prelude::*;
use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::{
    renet::{
        RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
        ServerEvent,
    },
    RenetServerPlugin,
};

use crate::{network::shared::ServerMessages, player::Player};

use crate::network::shared::Lobby;
pub struct ServerPlugin;

const PROTOCOL_ID: u64 = 7;

// let mut server = new_renet_server();

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin)
            .insert_resource(new_renet_server())
            .add_system(send_message_system)
            .add_system(receive_message_system)
            .add_system(handle_events_system)
            .insert_resource(Lobby::default());
    }
}

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // server.broadcast_message(channel_id, "server message".as_bytes().to_vec());
}
fn receive_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, channel_id)
        {
            println!("msg from client ");
            // Handle received message
        }
    }
}

fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                println!("Client {} connected", id);
                // spawn player
                let player_entity = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            ..Default::default()
                        },
                        transform: Transform {
                            scale: Vec3::new(16.0, 16.0, 1.0),
                            translation: Vec3::new(0.0, 0.0, 10.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player { id: *id })
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

                lobby.players.insert(*id, player_entity);

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
