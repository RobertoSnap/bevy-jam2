use bevy::prelude::*;

use crate::player::Player;
pub struct MovementPlugin;

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player);
    }
}

fn move_player(
    mut query: Query<(&mut Transform, &Movement), With<Player>>,
    time: Res<Time>,
) {
    const PLAYER_MOVE_SPEED: f32 = 20.;
    const PLAYER_JUMP_SPEED: f32 = 20.;
    for (mut transform, movement) in query.iter_mut() {
        if (movement.velocity == Vec3::ZERO) {
            return ();
        }
        println!("movement {:?}", movement.velocity);
        let direction = Vec3::new(
            movement.velocity.x * PLAYER_MOVE_SPEED * time.delta_seconds(),
            movement.velocity.y * PLAYER_MOVE_SPEED * time.delta_seconds(),
            movement.velocity.z * PLAYER_JUMP_SPEED * time.delta_seconds(),
        );
        transform.translation += direction;
    }
}
