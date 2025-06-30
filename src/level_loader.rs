use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use serde_json::from_slice;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct Chunk {
    pub data: Vec<i32>,
    pub x: i32,
    pub y: i32,
}

#[derive(Asset, TypePath, Debug, Deserialize)]
struct Layer {
    pub name: String,
    pub chunks: Vec<Chunk>,
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct Level {
    pub layers: Vec<Layer>,
}

#[derive(Resource, Default)]
pub struct LevelHandler(pub Handle<Level>);

#[derive(Default)]
pub struct LevelAssetLoader;

/// Possible errors that can be produced by [`LevelAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse JSON: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

impl AssetLoader for LevelAssetLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let asset = from_slice::<Level>(&bytes)?;

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
