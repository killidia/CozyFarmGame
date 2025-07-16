use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
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
        Name::new("player"),
    ));
}