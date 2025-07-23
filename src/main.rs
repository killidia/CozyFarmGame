use crate::player::PlayerPlugin;
use avian2d::prelude::PhysicsDebugPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use sprite_animation::SpriteAnimationPlugin;

mod map;
mod player;
mod sprite_animation;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            MapPlugin,
            PlayerPlugin,
            SpriteAnimationPlugin,
        ))
        .run();
}
