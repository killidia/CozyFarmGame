use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct TileMap;

#[derive(Default)]
struct RonAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum RonAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for RonAssetLoader {
    type Asset = TileMap;
    type Settings = ();
    type Error = RonAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<TileMap>(&bytes)?;

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
