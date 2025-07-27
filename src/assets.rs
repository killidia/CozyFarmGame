/*use bevy::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Chargement des textures
    let water_texture = asset_server.load("tilesets/water.png");
    let grass_texture = asset_server.load("tilesets/grass.png");
    let biome_texture = asset_server.load("tilesets/grass_biome.png");
    let player_texture = asset_server.load("characters/player.png");
    let player_action_texture = asset_server.load("characters/player_action.png");
    let tools_and_materials_texture = asset_server.load("tilesets/tools_and_materials.png");

    // Création des layouts
    let water_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
    let grass_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 11, 7, None, None);
    let biome_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 9, 5, None, None);
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let player_action_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 2, 10, None, None);
    let tools_and_materials_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 2, None, None);

    // Ajout des layouts aux assets
    let water_layout_handle = texture_atlas_layouts.add(water_layout);
    let grass_layout_handle = texture_atlas_layouts.add(grass_layout);
    let biome_layout_handle = texture_atlas_layouts.add(biome_layout);
    let player_layout_handle = texture_atlas_layouts.add(player_layout);
    let player_action_layout_handle = texture_atlas_layouts.add(player_action_layout);
    let tools_and_materials_layout_handle = texture_atlas_layouts.add(tools_and_materials_layout);

    // Insertion de la resource
    commands.insert_resource(GameAssets {
        water_texture,
        grass_texture,
        biome_texture,
        player_texture,
        player_action_texture,
        tools_and_materials_texture,
        water_layout: water_layout_handle,
        grass_layout: grass_layout_handle,
        biome_layout: biome_layout_handle,
        player_layout: player_layout_handle,
        player_action_layout: player_action_layout_handle,
        tools_and_materials_layout: tools_and_materials_layout_handle,
    });
}

// Resource contenant tous vos handles d'assets
#[derive(Resource)]
pub struct GameAssets {
    // Textures
    pub water_texture: Handle<Image>,
    pub grass_texture: Handle<Image>,
    pub biome_texture: Handle<Image>,
    pub player_texture: Handle<Image>,
    pub player_action_texture: Handle<Image>,
    pub tools_and_materials_texture: Handle<Image>,

    // Layouts des atlas de textures
    pub water_layout: Handle<TextureAtlasLayout>,
    pub grass_layout: Handle<TextureAtlasLayout>,
    pub biome_layout: Handle<TextureAtlasLayout>,
    pub player_layout: Handle<TextureAtlasLayout>,
    pub player_action_layout: Handle<TextureAtlasLayout>,
    pub tools_and_materials_layout: Handle<TextureAtlasLayout>,
}
*/
