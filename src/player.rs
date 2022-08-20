use bevy::prelude::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    name: String,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(16.0, 16.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player {
            name: "Ola".to_owned(),
        });
}
