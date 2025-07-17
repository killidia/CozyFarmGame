use std::time::Duration;

use crate::idle_animation::AnimationTimer;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone)]
enum PlayerDirection {
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, movement);
    }
}

fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Transform, &mut PlayerDirection), With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    // horizontal movement
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        *player.1 = PlayerDirection::Right;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        *player.1 = PlayerDirection::Left;
    }

    // vertical movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
        *player.1 = PlayerDirection::Up;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
        *player.1 = PlayerDirection::Down;
    }

    let moving = direction != Vec3::ZERO;

    if moving {
        // Normalize direction
        direction = direction.normalize();

        // apply player movement
        player.0.translation += direction * PLAYER_SPEED * time.delta_secs();
    }

    // player.3.0.tick(time.delta());
    //
    // if player.3.0.just_finished() {
    //     let player_direction = player.1.clone();
    //
    //     if let Some(atlas) = &mut player.2.texture_atlas {
    //         let index = match (player_direction, moving) {
    //             (PlayerDirection::Down, false) => if atlas.index == 1 { 0 } else { 1 },
    //             (PlayerDirection::Down, true) => if atlas.index == 3 { 2 } else { 3 },
    //
    //             (PlayerDirection::Up, false) => if atlas.index == 5 { 4 } else { 5 },
    //             (PlayerDirection::Up, true) => if atlas.index == 7 { 6 } else { 7 },
    //
    //             (PlayerDirection::Left, false) => if atlas.index == 9 { 8 } else { 9 },
    //             (PlayerDirection::Left, true) => if atlas.index == 11 { 10 } else { 11 },
    //
    //             (PlayerDirection::Right, false) => if atlas.index == 13 { 12 } else { 13 },
    //             (PlayerDirection::Right, true) => if atlas.index == 15 { 14 } else { 15 },
    //         };
    //         atlas.index = index;
    //     }
    // }
}
