use bevy::{ecs::bundle, prelude::*};
use bevy_mod_wanderlust::{
    CharacterControllerBundle, ControllerInput, ControllerPhysicsBundle,
    ControllerSettings, ControllerState,
};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::action_state::ActionDiff;

use crate::{
    input::Action,
    movement::Movement,
    network::shared::{Lobby, NetworkID},
};
pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCam;

#[derive(Component)]
pub struct Player;
#[derive(Bundle)]

pub struct PlayerBundle {
    pub name: Name,
    // pub gravity: GravityScale,
    // pub restitution: Restitution,
    // pub collider: Collider,
    // pub rigid_body: RigidBody,
    // pub velocity: Velocity,
    pub network_id: NetworkID,
    pub player: Player,
    #[bundle]
    pub pbr_bundle: PbrBundle,
    #[bundle]
    pub physics: ControllerPhysicsBundle,
    pub settings: ControllerSettings,
    pub input: ControllerInput,
    pub controller: ControllerState,
}

impl PlayerBundle {
    pub fn new(
        network_id: u64,
        mut mesh: Handle<Mesh>,
        mut material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            controller: ControllerState::default(),
            settings: ControllerSettings::default(),
            physics: ControllerPhysicsBundle::default(),
            input: ControllerInput::default(),
            // gravity: GravityScale(0.5),
            // restitution: Restitution::coefficient(0.7),
            // collider: Collider::cuboid(0.5, 0.5, 0.5),
            // rigid_body: RigidBody::KinematicPositionBased,
            // velocity: Velocity {
            //     linvel: Vec3::new(1.0, 2.0, 3.0),
            //     angvel: Vec3::new(0.2, 0.0, 0.0),
            // },
            name: Name::from("Player"),
            network_id: NetworkID(network_id),
            player: Player,
            pbr_bundle: PbrBundle {
                mesh: mesh,
                material: material,
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..Default::default()
            },
        }
    }
}

// This is the list of "things in the game I want to be able to do based on input"

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(actions);
    }
}

fn setup(mut commands: Commands) {
    //
}

fn actions(
    mut query: Query<&mut Velocity, With<Player>>,
    lobby: Res<Lobby>,
    mut events: EventReader<ActionDiff<Action, NetworkID>>,
    mut commands: Commands,
) {
    for mut event in events.iter() {
        match event {
            ActionDiff::Pressed { action, id } => {
                if let Some(player) = lobby.players.get(&id.0) {
                    // let mut movement = query
                    //     .get_mut(*player)
                    //     .expect("Moving player should have movement");
                    // match action {
                    //     Action::MoveLeft => {
                    //         println!("Player {} move left", id.0);
                    //         movement.velocity.x = -0.2;
                    //     }
                    //     Action::MoveRight => {
                    //         println!("Player {} move right", id.0);
                    //         movement.velocity.x = 0.2;
                    //     }
                    //     Action::MoveUp => {
                    //         println!("Player {} move up", id.0);
                    //         movement.velocity.z = -0.2;
                    //     }
                    //     Action::MoveDown => {
                    //         println!("Player {} move down", id.0);
                    //         movement.velocity.z = 0.2;
                    //     }
                    //     Action::Jump => {
                    //         println!("Player {} jumped", id.0);
                    //         movement.velocity.y = 0.5;
                    //     }
                    //     Action::Shoot => {
                    //         println!("Player {} shoot", id.0);
                    //     }
                    // };
                }
            }
            ActionDiff::Released { action, id } => {
                if let Some(player) = lobby.players.get(&id.0) {
                    // let mut movement = query
                    //     .get_single_mut()
                    //     .expect("Moving player should have movement");

                    // match action {
                    //     Action::MoveLeft => {
                    //         println!("Player {} STOP move left", id.0);
                    //         movement.velocity.x = 0.0;
                    //     }
                    //     Action::MoveRight => {
                    //         println!("Player {} STOP move right", id.0);
                    //         movement.velocity.x = 0.0;
                    //     }
                    //     Action::MoveUp => {
                    //         println!("Player {} STOP move up", id.0);
                    //         movement.velocity.z = 0.0;
                    //     }
                    //     Action::MoveDown => {
                    //         println!("Player {} STOP move down", id.0);
                    //         movement.velocity.z = 0.0;
                    //     }
                    //     Action::Jump => {
                    //         println!("Player {} STOP jumped", id.0);
                    //         movement.velocity.y = 0.0;
                    //     }
                    //     Action::Shoot => {
                    //         println!("Player {} STOP shoot", id.0);
                    //     }
                    // };
                }
            }
        }
    }
    // let action_state = query.iter().next();
    // // Each action has a button-like state of its own that you can check
    // if let Some(action_state) = action_state {
    //     if action_state.just_pressed(Action::Jump) {
    //         println!("I'm jumping!");
    //     }
    // }
}
