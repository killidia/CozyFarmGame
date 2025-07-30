use crate::player::{Player, PlayerState, Tool};
use crate::sprite_animation::{AnimationIndices, FrameTimer};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use std::fmt::Debug;

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
        app.add_systems(Startup, load_map)
            .add_systems(Update, (till_after_timer, growing_after_timer));
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
                commands
                    .spawn((
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
                        Tillable,
                        Pickable::default(),
                        Name::new("grass"),
                    ))
                    .observe(till_grass);
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
                            10.0,
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
                    commands.entity(tile_entity).insert((
                        RigidBody::Static,
                        Collider::circle(6.0),
                        Rock,
                    ));
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

fn till_grass(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut player: Single<(&Transform, &mut Player)>,
    grass_tiles: Query<(Entity, &Transform)>,
) {
    let Ok((grass_entity, grass_transform)) = grass_tiles.get(trigger.target()) else {
        return;
    };

    if player.1.current_tool == Tool::Hoe
        && player.0.translation.distance(grass_transform.translation) <= 24.0
    {
        player.1.state = PlayerState::Tilling;

        commands
            .entity(grass_entity)
            .insert(TilledTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    }
}

fn till_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Single<&mut Player>,
    mut grass_tiles: Query<(Entity, &Transform, &mut TilledTimer)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, transform, mut tilled_timer) in &mut grass_tiles {
        tilled_timer.0.tick(time.delta());

        if tilled_timer.0.just_finished() {
            player.state = PlayerState::Idle;

            // replace grass tile by dirt tile
            commands.entity(entity).despawn();

            let dirt_texture = asset_server.load("tilesets/dirt.png");
            let dirt_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 11, 7, None, None);
            let dirt_texture_atlas_layout = texture_atlas_layouts.add(dirt_layout);

            commands
                .spawn((
                    Sprite::from_atlas_image(
                        dirt_texture,
                        TextureAtlas {
                            layout: dirt_texture_atlas_layout,
                            index: 12,
                        },
                    ),
                    Transform::from_translation(transform.translation),
                    Seedable,
                    Pickable::default(),
                    Name::new("dirt"),
                ))
                .observe(seed_dirt);
        }
    }
}

fn seed_dirt(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    player: Single<(&Transform, &Player)>,
    dirt_tiles: Query<(Entity, &Transform)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok((dirt_entity, dirt_transform)) = dirt_tiles.get(trigger.target()) else {
        return;
    };

    if player.1.current_tool == Tool::Hoe
        && player.0.translation.distance(dirt_transform.translation) <= 24.0
    {
        let texture = asset_server.load("tilesets/plants.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 7,
                },
            ),
            Transform::from_xyz(
                dirt_transform.translation.x,
                dirt_transform.translation.y,
                3.0,
            ),
            EndGrowing(10),
            GrowingTimer(Timer::from_seconds(15.0, TimerMode::Repeating)),
            DirtTile(dirt_entity),
            Name::new("plant"),
        ));

        commands.entity(dirt_entity).remove::<Seedable>();
    }
}

fn growing_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut growings: Query<(
        Entity,
        &mut GrowingTimer,
        &EndGrowing,
        &mut Sprite,
        &DirtTile,
    )>,
) {
    for (entity, mut growing_timer, growing_indices, mut sprite, dirt_tile) in &mut growings {
        growing_timer.0.tick(time.delta());

        if growing_timer.0.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                if atlas.index == growing_indices.0 {
                    // despawn entity
                    commands.entity(entity).despawn();

                    // mark dirt tile seedable
                    commands.entity(dirt_tile.0).insert(Seedable);
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Rock;

#[derive(Component, Debug)]
struct Tillable;

#[derive(Component)]
struct TilledTimer(Timer);

#[derive(Component)]
struct Seedable;

#[derive(Component)]
struct DirtTile(Entity);

#[derive(Component)]
struct EndGrowing(usize);

#[derive(Component)]
struct GrowingTimer(Timer);
