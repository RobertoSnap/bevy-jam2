use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, run_if_client_connected};
use leafwing_input_manager::{action_state::ActionDiff, prelude::ActionState};

use crate::{
    input::Action,
    network::shared::{PlayerID, ServerMessages},
};
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub id: u64,
}
// This is the list of "things in the game I want to be able to do based on input"

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(jump.with_run_criteria(run_if_client_connected));
    }
}

fn setup(mut commands: Commands) {
    //
}

fn jump(
    query: Query<&ActionState<Action>, With<Player>>,
    mut client: ResMut<RenetClient>,
    player_id: Res<PlayerID>,
) {
    let action_state = query.iter().next();
    // Each action has a button-like state of its own that you can check
    if let Some(action_state) = action_state {
        if action_state.just_pressed(Action::Jump) {
            println!("I'm jumping!");
        }
    }
}
