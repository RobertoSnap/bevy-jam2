use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, run_if_client_connected};
use leafwing_input_manager::{action_state::ActionDiff, prelude::ActionState};

use crate::{
    input::Action,
    network::shared::{Lobby, NetworkID},
};
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub id: u64,
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
    mut query: Query<&mut Transform, (With<Player>, With<NetworkID>)>,
    lobby: Res<Lobby>,
    mut events: EventReader<ActionDiff<Action, NetworkID>>,
) {
    for mut event in events.iter() {
        if let ActionDiff::Pressed { action, id } = event {
            if let Some(player) = lobby.players.get(&id.0) {
                println!("got player");
                if let Ok(mut transform) = query.get_single_mut() {
                    match action {
                        Action::MoveLeft => {
                            println!("Player {} move left", id.0);
                            transform.translation.x -= 5.;
                        }
                        Action::MoveRight => {
                            println!("Player {} move right", id.0);
                            transform.translation.x += 5.;
                        }
                        Action::Jump => {
                            println!("Player {} jumped", id.0);
                        }
                        Action::Shoot => {
                            println!("Player {} shoot", id.0);
                        }
                    };
                }
            };
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
