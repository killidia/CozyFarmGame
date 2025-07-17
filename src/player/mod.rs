mod animation;
mod movement;

use crate::player::animation::{PlayerAnimation, PlayerAnimationPlugin};
use crate::player::movement::{MovementController, MovementPlugin};
use crate::AppSystems;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, PlayerAnimationPlugin))
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                record_player_directional_input.in_set(AppSystems::RecordInput),
            );
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

    commands.spawn((
        Sprite::from_atlas_image(
            player_texture,
            TextureAtlas {
                layout: player_texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_xyz(32.0, -32.0, 3.0),
        Player,
        MovementController::default(),
        PlayerAnimation::new(),
        Name::new("Player"),
    ));
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Component)]
struct Player;
