use crate::inventory::PlayerInventory;
use crate::map::Rock;
use crate::player::{Player, PlayerState, Tool};
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
                Transform::from_xyz(transform.translation.x, transform.translation.y, 4.0),
                Name::new("Collectible rock"),
            ));

            player.0.state = PlayerState::Idle;
        }
    }
}

fn collect_item(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    collectibles: Query<(Entity, &Transform, &Collectible)>,
    mut player_inventory: ResMut<PlayerInventory>,
) {
    for (entity, transform, collectible) in collectibles {
        if player.translation.distance(transform.translation) <= 3.0 {
            if collectible.item == CollectibleType::Rock {
                player_inventory.rock += 1;
            }

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

#[derive(PartialEq)]
enum CollectibleType {
    Rock,
}
