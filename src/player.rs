use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.0;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(Update, movement);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let player_texture = asset_server.load("characters/player.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

    commands.spawn((
        Sprite::from_atlas_image(
            player_texture,
            TextureAtlas {
                layout: player_texture_atlas_layout,
                index: 0
            }
        ),
        Transform::from_xyz(32.0, -32.0, 3.0),
        Player,
        Name::new("player"),
    ));
}

fn movement(keyboard_input: Res<ButtonInput<KeyCode>>, mut player_transform: Single<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut direction = Vec3::ZERO;

    // horizontal movement
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    // vertical movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    // Normalize direction
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    // apply player movement
    player_transform.translation += direction * PLAYER_SPEED * time.delta_secs();
}