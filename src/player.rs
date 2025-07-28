use crate::sprite_animation::{AnimationIndices, FrameTimer};
use avian2d::prelude::{
    Collider, CollidingEntities, GravityScale, LinearVelocity, LockedAxes, RigidBody,
};
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
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 10, 4, None, None);
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
            Transform::from_xyz(32.0, -32.0, 4.0),
            Player {
                current_direction: PlayerDirection::Down,
                state: PlayerState::default(),
                current_tool: Tool::Axe,
            },
            AnimationIndices { first: 0, last: 1 },
            FrameTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            RigidBody::Dynamic,
            Collider::circle(6.0),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),
            CollidingEntities::default(),
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
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut LinearVelocity, &mut Player)>,
) {
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;

        player.1.current_direction = PlayerDirection::Up;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;

        player.1.current_direction = PlayerDirection::Down;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;

        player.1.current_direction = PlayerDirection::Left;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;

        player.1.current_direction = PlayerDirection::Right;
    }

    // Normalize direction so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    if direction != Vec2::ZERO {
        direction = direction.normalize();
        player.1.state = PlayerState::Walking;

        player.0.0 = direction * PLAYER_SPEED;
    } else {
        if player.1.state == PlayerState::Walking {
            player.1.state = PlayerState::Idle;
        }

        player.0.0 = Vec2::ZERO;
    }
}

fn update_indices(mut query: Query<(&mut AnimationIndices, &mut Sprite, &Player)>) {
    for (mut indices, mut sprite, player) in &mut query {
        let new_indices = player_sprite_indices(&player.state, &player.current_direction);

        if new_indices.0 != indices.first {
            indices.first = new_indices.0;
            indices.last = new_indices.1;

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = indices.first;
            }
        }
    }
}

pub fn player_sprite_indices(state: &PlayerState, direction: &PlayerDirection) -> (usize, usize) {
    match state {
        PlayerState::Idle => match direction {
            PlayerDirection::Right => (30, 31),
            PlayerDirection::Left => (20, 21),
            PlayerDirection::Up => (10, 11),
            _ => (0, 1),
        },
        PlayerState::Walking => match direction {
            PlayerDirection::Right => (32, 33),
            PlayerDirection::Left => (22, 23),
            PlayerDirection::Up => (12, 13),
            _ => (2, 3),
        },
        PlayerState::Chopping => match direction {
            PlayerDirection::Right => (37, 37),
            PlayerDirection::Left => (26, 27),
            PlayerDirection::Up => (16, 17),
            _ => (6, 7),
        },
    }
}

#[derive(Default, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Chopping,
}

#[derive(Component)]
pub struct Player {
    pub current_direction: PlayerDirection,
    pub state: PlayerState,
    pub current_tool: Tool,
}

pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum Tool {
    Axe, /*Hoe,
         None*/
}
