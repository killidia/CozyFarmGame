use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct Level {
    pub data: Vec<Vec<i8>>,
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
    #[error("Could not parse CSV: {0}")]
    CsvError(#[from] csv::Error),
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

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_reader(bytes.as_slice());

        let mut rows = vec![];

        for result in rdr.deserialize() {
            rows.push(result?);
        }

        Ok(Level { data: rows })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
