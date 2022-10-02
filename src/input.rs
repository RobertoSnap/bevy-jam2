use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};
pub struct InputPlugin;

#[derive(
    Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize,
)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Jump,
    Shoot,
}

// /// This identifier uniquely identifies entities across the network
// #[derive(Component, Clone, PartialEq, Eq, Debug)]
// pub struct StableId(pub u64);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}
