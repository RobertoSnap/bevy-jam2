use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
pub struct InputPlugin;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    Shoot,
}

/// This identifier uniquely identifies entities across the network
#[derive(Component, Clone, PartialEq, Eq, Debug)]
pub struct StableId(u64);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}
