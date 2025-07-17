use std::time::Duration;

use crate::player::movement::MovementController;
use crate::AppSystems;
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_animation_timer.in_set(AppSystems::TickTimers),
                (update_animation_movement, update_animation_atlas)
                    .chain()
                    .in_set(AppSystems::Update),
            ),
        );
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, _sprite, mut animation) in &mut player_query {
        let animation_state = if controller.intent == Vec2::ZERO {
            PlayerAnimationState::Idle
        } else {
            PlayerAnimationState::Walk
        };

        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

#[derive(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(PartialEq)]
enum PlayerAnimationState {
    Idle,
    Walk,
}

impl PlayerAnimation {
    // The number of idle frames
    const ANIMATION_FRAMES: usize = 2;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(300);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idle,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walk,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    // Update animation timer
    fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if !self.timer.finished() {
            return;
        }

        self.frame = (self.frame + 1) % Self::ANIMATION_FRAMES;
    }

    // Update animation state if it changes
    fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idle => *self = Self::idling(),
                PlayerAnimationState::Walk => *self = Self::walking(),
            }
        }
    }

    // Whether animation changed this tick.
    fn changed(&self) -> bool {
        self.timer.finished()
    }

    fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idle => self.frame,
            PlayerAnimationState::Walk => 14 + self.frame,
        }
    }
}
