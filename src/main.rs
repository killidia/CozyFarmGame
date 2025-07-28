use crate::biome_interaction::BiomeInteractionPlugin;
use crate::player::PlayerPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use sprite_animation::SpriteAnimationPlugin;

mod biome_interaction;
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
            MapPlugin,
            PlayerPlugin,
            SpriteAnimationPlugin,
            BiomeInteractionPlugin,
        ))
        .run();
}
