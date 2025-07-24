use crate::map::Rock;
use crate::player::Player;
use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;

pub struct BiomeInteractionPlugin;

impl Plugin for BiomeInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (remove_biome_element, remove_after_timer));
    }
}

fn remove_biome_element(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    colliding_query: Single<(&CollidingEntities, &mut Player)>,
    rocks: Query<Entity, With<Rock>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        let colliding_entities = colliding_query.into_inner();

        // Fetch all entites in collision with player
        for &colliding_entity in colliding_entities.0.iter() {
            if rocks.get(colliding_entity).is_ok() {
                commands
                    .entity(colliding_entity)
                    .insert(RemoveTimer(Timer::from_seconds(3.0, TimerMode::Once)));

                // TODO: play axe animation (require another player spritesheet)
                // Todo: check if player has axe in inventory (require Inventory Resource for player)

                break;
            }
        }
    }
}

fn remove_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut RemoveTimer)>,
) {
    for (entity, mut timer) in &mut timers {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.entity(entity).despawn();

            // TODO: drop collectible rock (load tools_and_materials.png spritesheet)
        }
    }
}

#[derive(Component)]
struct RemoveTimer(Timer);
