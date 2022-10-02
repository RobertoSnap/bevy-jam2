use bevy::prelude::*;
use bevy_framepace;
use bevy_mod_wanderlust::{
    CharacterControllerBundle, ControllerInput, WanderlustPhysicsTweaks,
    WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;
use std::time::Duration;

mod input;
mod map;
mod movement;
mod player;
mod network {
    pub mod client;
    pub mod error;
    pub mod server;
    pub mod shared;
}

pub const LAUNCHER_TITLE: &str = "Jam2";

#[derive(Reflect)]
struct Sensitivity(f32);

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
    // .add_plugin(bevy_framepace::FramepacePlugin)
    // .insert_resource(bevy_framepace::FramepaceSettings {
    //     limiter: bevy_framepace::Limiter::Manual(Duration::from_millis(30)),
    // })
    // .add_startup_system(setup_graphics)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(WanderlustPlugin)
    .insert_resource(Sensitivity(1.0))
    .add_plugin(map::MapPlugin)
    .add_plugin(input::InputPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(movement::MovementPlugin);
    if is_host {
        app.add_plugin(network::server::ServerPlugin)
            .add_startup_system(server_camera);
        println!("Server RUNNING");
    } else {
        app.add_plugin(network::client::ClientPlugin);
        println!("Client RUNNING");
    }

    app.add_system(network::error::panic_on_error_system);

    app
}
// fn setup_graphics(mut commands: Commands) {
//     // Add a camera so we can see the debug-render.
//     commands.spawn_bundle(PerspectiveCameraBundle {
//         transform: Transform::from_xyz(-3.0, 3.0, 10.0)
//             .looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
// }

fn server_camera(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-8., 8., 20.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
