use crate::map::Rock;
use crate::player::{player_sprite_indices, Player, PlayerState, Tool};
use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;

pub struct BiomeInteractionPlugin;

impl Plugin for BiomeInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (remove_biome_element, remove_after_timer, collect_item),
        );
    }
}

fn remove_biome_element(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    colliding_query: Single<(&CollidingEntities, &mut Player, &mut Sprite)>,
    rocks: Query<Entity, With<Rock>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        let mut colliding_entities = colliding_query.into_inner();

        // Fetch all entites in collision with player
        for &colliding_entity in colliding_entities.0.iter() {
            if rocks.get(colliding_entity).is_ok() && colliding_entities.1.current_tool == Tool::Axe
            {
                commands
                    .entity(colliding_entity)
                    .insert(RemoveTimer(Timer::from_seconds(3.0, TimerMode::Once)));

                colliding_entities.1.state = PlayerState::Chopping;

                let texture = asset_server.load("characters/player_action.png");
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 2, 10, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                *colliding_entities.2 = Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: player_sprite_indices(
                            &colliding_entities.1.state,
                            &colliding_entities.1.current_direction,
                        )
                        .0,
                    },
                );

                break;
            }
        }
    }
}

fn remove_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &Transform, &mut RemoveTimer)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player: Single<(&mut Player, &mut Sprite)>,
) {
    for (entity, transform, mut timer) in &mut timers {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.entity(entity).despawn();

            let texture = asset_server.load("tilesets/tools_and_materials.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 2, None, None);
            let atlas_layout = texture_atlas_layouts.add(layout);

            commands.spawn((
                Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: atlas_layout,
                        index: 3,
                    },
                ),
                Collectible {
                    item: CollectibleType::Rock,
                },
                Transform::from_xyz(transform.translation.x, transform.translation.y, 3.0),
                Name::new("Collectible rock"),
            ));

            player.0.state = PlayerState::Idle;

            // reset player
            let player_texture = asset_server.load("characters/player.png");
            let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
            let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

            *player.1 = Sprite::from_atlas_image(
                player_texture,
                TextureAtlas {
                    layout: player_texture_atlas_layout,
                    index: 0,
                },
            );
        }
    }
}

fn collect_item(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    collectibles: Query<(Entity, &Transform), With<Collectible>>,
) {
    for (entity, transform) in collectibles {
        if player.translation.distance(transform.translation) <= 3.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct RemoveTimer(Timer);

#[derive(Component)]
struct Collectible {
    item: CollectibleType,
}

enum CollectibleType {
    Rock,
}
