use bevy::prelude::*;

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 3.0;
const SCALED_TILE_SIZE: f32 = TILE_SIZE * TILE_SCALE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map);
    }
}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // spawn water layer
    let water_tileset = asset_server.load("tilesets/Water.png");
    let water_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);

    let water_atlas_layout = texture_atlas_layouts.add(water_layout);

    for x in 0..60 {
        for y in 0..40 {
            commands.spawn((
                Sprite::from_atlas_image(
                    water_tileset.clone(),
                    TextureAtlas {
                        layout: water_atlas_layout.clone(),
                        index: 0,
                    },
                ),
                Transform {
                    translation: Vec3::new(
                        x as f32 * SCALED_TILE_SIZE,
                        y as f32 * SCALED_TILE_SIZE,
                        1.0,
                    ),
                    scale: Vec3::splat(TILE_SCALE),
                    ..default()
                },
            ));
        }
    }
}
