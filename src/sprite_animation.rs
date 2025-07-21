use bevy::prelude::*;

pub struct SpriteAnimation;

impl Plugin for SpriteAnimation {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component, Default)]
struct AnimSprite {
    sprites: usize,
    repeating: bool,
    disable: bool,
}

#[derive(Component)]
struct AnimSpriteTimer {
    timer: Timer,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct FrameTimer(Timer);

impl AnimSprite {
    fn new(sprites: usize, repeating: bool) -> Self {
        Self {
            sprites: sprites - 1,
            repeating,
            disable: false,
        }
    }
}

impl Default for AnimSpriteTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

impl AnimSpriteTimer {
    fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}
