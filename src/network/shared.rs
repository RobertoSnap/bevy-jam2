use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
    PlayerJump { id: u64 },
}

#[derive(Debug, Default)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}
#[derive(
    Component, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct NetworkID(pub u64);
