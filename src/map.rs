use crate::sprite_animation::{AnimationIndices, FrameTimer};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

const TILE_SIZE: u8 = 16;

const WATER_LAYER: [[i8; 10]; 10] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const GRASS_LAYER: [[i8; 10]; 10] = [
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, 0, 1, 1, 1, 1, 1, 1, 2, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 11, 12, 12, 12, 12, 12, 12, 13, -1],
    [-1, 22, 23, 23, 23, 23, 23, 23, 24, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
];

const BIOME_LAYER: [[i8; 10]; 10] = [
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, 3, 4, -1, -1, -1, -1, -1],
    [-1, -1, -1, 12, 13, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, 0, -1, -1],
    [-1, -1, -1, -1, 17, -1, -1, 9, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
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
    let biome_texture = asset_server.load("tilesets/grass_biome.png");

    let water_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 1, None, None);
    let grass_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 11, 7, None, None);
    let biome_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 9, 5, None, None);

    let water_texture_atlas_layout = texture_atlas_layouts.add(water_layout);
    let grass_texture_atlas_layout = texture_atlas_layouts.add(grass_layout);
    let biome_texture_atlas_layout = texture_atlas_layouts.add(biome_layout);

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
                FrameTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
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

    // spawn grass biome layer
    for (y, row) in BIOME_LAYER.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile >= &0 {
                let tile_entity = commands
                    .spawn((
                        Sprite::from_atlas_image(
                            biome_texture.clone(),
                            TextureAtlas {
                                layout: biome_texture_atlas_layout.clone(),
                                index: *tile as usize,
                            },
                        ),
                        Transform::from_xyz(
                            x as f32 * TILE_SIZE as f32,
                            -(y as f32 * TILE_SIZE as f32),
                            4.0,
                        ),
                        Name::new("Grass Biome"),
                    ))
                    .id();

                // add collision for some tiles
                if tile == &9 {
                    commands
                        .entity(tile_entity)
                        .insert((RigidBody::Static, Collider::rectangle(8.0, 8.0)));
                } else if tile == &17 {
                    commands
                        .entity(tile_entity)
                        .insert((RigidBody::Static, Collider::circle(6.0)));
                } else if tile == &12 {
                    // Créer une entité collider séparée décalée vers la gauche
                    commands.spawn((
                        RigidBody::Static,
                        Collider::rectangle(4.0, 8.0),
                        Transform::from_xyz(
                            x as f32 * TILE_SIZE as f32 + 6.0, // Position de base - décalage gauche
                            -(y as f32 * TILE_SIZE as f32) - 2.0,
                            4.0,
                        ),
                        Name::new("Collider Tile 12"),
                    ));
                } else if tile == &13 {
                    // Créer une entité collider séparée décalée vers la droite
                    commands.spawn((
                        RigidBody::Static,
                        Collider::rectangle(4.0, 8.0),
                        Transform::from_xyz(
                            x as f32 * TILE_SIZE as f32 - 6.0, // Position de base + décalage droite
                            -(y as f32 * TILE_SIZE as f32) - 2.0,
                            4.0,
                        ),
                        Name::new("Collider Tile 13"),
                    ));
                }
            }
        }
    }
}
