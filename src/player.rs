use bevy::prelude::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub id: u64,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    //
}
