use bevy::prelude::*;
use bevy_framepace;
mod input;
mod map;
mod player;
mod network {
    pub mod client;
    pub mod error;
    pub mod server;
    pub mod shared;
}

pub const LAUNCHER_TITLE: &str = "Jam2";

pub fn app() -> App {
    println!("Usage: run with \"server\" or \"client\" argument");
    let args: Vec<String> = std::env::args().collect();

    let exec_type = &args[1];
    let is_host = match exec_type.as_str() {
        "client" => false,
        "server" => true,
        _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
    };
    let server_string = match is_host {
        true => "server",
        false => "client",
    };
    println!("is_host: {}", is_host);
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: vec![LAUNCHER_TITLE, server_string].join(" "),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        position: WindowPosition::At(Vec2::new(
            0.,
            if is_host { 0. } else { 400. },
        )),
        height: 270.,
        width: 480.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_framepace::FramepacePlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(input::InputPlugin)
    .add_plugin(player::PlayerPlugin);
    if is_host {
        app.add_plugin(network::server::ServerPlugin);
        println!("Server RUNNING");
    } else {
        app.add_plugin(network::client::ClientPlugin);
        println!("Client RUNNING");
    }
    app.add_system(network::error::panic_on_error_system);

    app
}
