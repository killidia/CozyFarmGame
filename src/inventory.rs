use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInventory>();
    }
}

#[derive(Default, Debug, Resource)]
pub struct PlayerInventory {
    pub rock: u8,
}
