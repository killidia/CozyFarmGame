use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, movement);
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
        Name::new("Player"),
    ));
}

fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
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
    }

    player_transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
}

#[derive(Component)]
struct Player;
