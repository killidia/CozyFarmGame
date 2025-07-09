use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use map::MapPlugin;

mod map;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TiledMapPlugin::default(),
            MapPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.3,
            ..OrthographicProjection::default_2d()
        })
    ));
}
