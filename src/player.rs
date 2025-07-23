use crate::sprite_animation::{AnimationIndices, FrameTimer};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (movement, update_indices));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_texture = asset_server.load("characters/player.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

    commands
        .spawn((
            Sprite::from_atlas_image(
                player_texture,
                TextureAtlas {
                    layout: player_texture_atlas_layout,
                    index: 0,
                },
            ),
            Transform::from_xyz(32.0, -32.0, 3.0),
            Player {
                current_direction: Vec2::ZERO,
                state: PlayerState::default(),
            },
            AnimationIndices { first: 0, last: 1 },
            FrameTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Name::new("Player"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera2d,
                Projection::Orthographic(OrthographicProjection {
                    scale: 0.3,
                    ..OrthographicProjection::default_2d()
                }),
            ));
        });
}

fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Transform, &mut Player)>,
) {
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // Normalize direction so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    if direction != Vec2::ZERO {
        direction = direction.normalize();
        player.1.state = PlayerState::Walking;
    } else {
        player.1.state = PlayerState::Idle;
    }

    player.0.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    player.1.current_direction = direction;
}

fn update_indices(mut query: Query<(&mut AnimationIndices, &mut Sprite, &Player)>) {
    for (mut indices, mut sprite, player) in &mut query {
        let new_indices = player_sprite_indices(&player.state, player.current_direction);

        if new_indices.0 != indices.first {
            indices.first = new_indices.0;
            indices.last = new_indices.1;

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.first;
            }
        }
    }
}

fn player_sprite_indices(state: &PlayerState, direction: Vec2) -> (usize, usize) {
    match state {
        PlayerState::Idle => {
            match direction {
                Vec2::X => (12, 13),   // Right
                Vec2::NEG_X => (8, 9), // Left
                Vec2::Y => (4, 5),     // Up
                _ => (0, 1),           // Down
            }
        }
        PlayerState::Walking => {
            match direction {
                Vec2::X => (14, 15),     // Right
                Vec2::NEG_X => (10, 11), // Left
                Vec2::Y => (6, 7),       // Up
                _ => (2, 3),             // Down
            }
        }
    }
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    Walking,
}

#[derive(Component)]
struct Player {
    current_direction: Vec2,
    state: PlayerState,
}
