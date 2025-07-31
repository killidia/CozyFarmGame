use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("ui/ui.png");
    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, Some(UVec2::splat(8)));
    let atlas_layout_handle = texture_atlases.add(atlas_layout);

    // index: 7

    let slicer = TextureSlicer {
        border: BorderRect::all(8.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

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

            builder
                .spawn(Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((ImageNode::from_atlas_image(
                        texture,
                        TextureAtlas {
                            index: 7,
                            layout: atlas_layout_handle,
                        },
                    ),));
                });
        });
}
