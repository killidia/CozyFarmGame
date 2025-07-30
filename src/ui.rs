use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui = asset_server.load("ui/ui.png");

    // TODO: splice image to display UI

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Grid,
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::auto()],
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Node {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::auto(),
                ],
                ..default()
            });

            builder.spawn(Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            });
        });
}
