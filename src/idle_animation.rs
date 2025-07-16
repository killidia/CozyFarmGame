use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct IdleAnimationPlugin;

impl Plugin for IdleAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, idle_animation);
    }
}

fn idle_animation(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atals) = &mut sprite.texture_atlas {
                atals.index = if atals.index == indices.last {
                    indices.first
                } else {
                    atals.index + 1
                }
            }
        }
    }
}