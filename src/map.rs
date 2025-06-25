use bevy::prelude::*;

const TILE_SIZE: u32 = 16;
const TILE_SCALE: f32 = 3.0;
const SCALED_TILE_SIZE: f32 = TILE_SIZE as f32 * TILE_SCALE;

#[derive(Component, Clone, Copy)]
struct WaterAnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct WaterAnimationTimer(Timer);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map)
            .add_systems(Update, water_animation);
    }
}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // spawn water layer
    let water_tileset = asset_server.load("tilesets/Water.png");
    let water_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);

    let water_atlas_layout = texture_atlas_layouts.add(water_layout);

    let water_animation_indices = WaterAnimationIndices { first: 0, last: 3 };

    for x in 0..60 {
        for y in 0..40 {
            commands.spawn((
                Sprite::from_atlas_image(
                    water_tileset.clone(),
                    TextureAtlas {
                        layout: water_atlas_layout.clone(),
                        index: water_animation_indices.first,
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
                water_animation_indices,
                WaterAnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            ));
        }
    }
}

fn water_animation(
    time: Res<Time>,
    mut query: Query<(
        &WaterAnimationIndices,
        &mut WaterAnimationTimer,
        &mut Sprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
