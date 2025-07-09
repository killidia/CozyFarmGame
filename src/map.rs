use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TiledMapHandle, TilemapAnchor};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map);
    }
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map/main.tmx");

    commands.spawn((TiledMapHandle(map_handle), TilemapAnchor::Center));
}
