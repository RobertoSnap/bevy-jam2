use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::input::Action;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub id: u64,
}
// This is the list of "things in the game I want to be able to do based on input"

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(jump);
    }
}

fn setup(mut commands: Commands) {
    //
}

fn jump(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.iter().next();
    // Each action has a button-like state of its own that you can check
    if let Some(action_state) = action_state {
        if action_state.just_pressed(Action::Jump) {
            println!("I'm jumping!");
        }
    }
}
