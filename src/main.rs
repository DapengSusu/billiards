//! Renders a 2D scene containing a single, moving sprite.

// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    let height = 450.0;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: height * ASPECT_RATIO,
                height,
                position: WindowPosition::Automatic,
                title: "bevy_game".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("textures\\bevy.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Direction::Down);
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y - 200.0 > 0.0001 {
            *logo = Direction::Down;
        } else if transform.translation.y + 200.0 < -0.0001 {
            *logo = Direction::Up;
        }
    }
}
