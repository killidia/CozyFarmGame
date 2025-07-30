use crate::biome_interaction::BiomeInteractionPlugin;
use crate::inventory::InventoryPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use sprite_animation::SpriteAnimationPlugin;

mod biome_interaction;
mod inventory;
mod map;
mod player;
mod sprite_animation;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            MapPlugin,
            PlayerPlugin,
            InventoryPlugin,
            SpriteAnimationPlugin,
            BiomeInteractionPlugin,
            UiPlugin,
        ))
        .run();
}
