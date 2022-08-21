use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin, RenetServerPlugin,
};
pub struct ClientPlugin;

const PROTOCOL_ID: u64 = 7;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            .insert_resource(new_renet_client())
            .add_system(
                client_send_input.with_run_criteria(run_if_client_connected),
            );
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

fn client_send_input(mut client: ResMut<RenetClient>) {
    // let input_message = bincode::serialize(&*player_input).unwrap();
    while let Some(message) = client.receive_message(0) {
        // let server_message = bincode::deserialize(&message).unwrap();
        println!("Client rcv msg");
    }
    // client.send_message(0, "input_message".as_bytes().to_vec());
}
