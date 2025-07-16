use bevy::prelude::*;
use crate::idle_animation::{AnimationIndices, AnimationTimer};

const TILE_SIZE: u8 = 16;

const WATER_LAYER: [[i8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const GRASS_LAYER: [[i8; 5]; 5] = [
    [-1, -1, -1, -1, -1],
    [-1, 0, 1, 2, -1],
    [-1, 11, 12, 13, -1],
    [-1, 22, 23, 24, -1],
    [-1, -1, -1, -1, -1],
];

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
    let water_texture = asset_server.load("tilesets/water.png");
    let grass_texture = asset_server.load("tilesets/grass.png");

    let water_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 1, None, None);
    let grass_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 11, 7, None, None);

    let water_texture_atlas_layout = texture_atlas_layouts.add(water_layout);
    let grass_texture_atlas_layout = texture_atlas_layouts.add(grass_layout);

    // spawn water layers
    for (y, row) in WATER_LAYER.iter().enumerate() {
        for (x, _tile) in row.iter().enumerate() {
            commands.spawn((
                Sprite::from_atlas_image(
                    water_texture.clone(),
                    TextureAtlas {
                        layout: water_texture_atlas_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE as f32,
                    -(y as f32 * TILE_SIZE as f32),
                    1.0,
                ),
                AnimationIndices { first: 0, last: 3 },
                AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
                Name::new("water"),
            ));
        }
    }

    // spawn grass layers
    for (y, row) in GRASS_LAYER.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile >= &0 {
                commands.spawn((
                    Sprite::from_atlas_image(
                        grass_texture.clone(),
                        TextureAtlas {
                            layout: grass_texture_atlas_layout.clone(),
                            index: *tile as usize,
                        },
                    ),
                    Transform::from_xyz(
                        x as f32 * TILE_SIZE as f32,
                        -(y as f32 * TILE_SIZE as f32),
                        2.0,
                    ),
                    Name::new("grass"),
                ));
            }
        }
    }
}