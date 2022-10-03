use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::{BoxedFuture, HashMap};
use thiserror::Error;

pub type DatasetData = HashMap<String, Vec<[f32; 2]>>;

#[derive(Debug, TypeUuid)]
#[uuid = "b07671ba-d0ce-4ac3-b028-7d05fd890e7a"]
pub struct Dataset {
    pub data: DatasetData,
}

impl Dataset {
    pub fn from_buffer(buffer: &[u8]) -> Result<Dataset, DatasetError> {
        let text = std::str::from_utf8(buffer)?;
        let mut data = HashMap::new();
        for (idx, line) in text.lines().enumerate() {
            let idx = idx + 1;
            let line = line.trim();
            if line.is_empty() || line.chars().next().unwrap() == '#' {
                continue;
            }

            let mut itr = line.split_whitespace();
            let x: f32 = itr
                .next()
                .ok_or(DatasetError::MissingArgumentError {
                    idx,
                    text: line.to_string(),
                })?
                .replace(',', ".")
                .parse()?;

            let y: f32 = itr
                .next()
                .ok_or(DatasetError::MissingArgumentError {
                    idx,
                    text: line.to_string(),
                })?
                .replace(',', ".")
                .parse()?;

            let g = itr
                .next()
                .ok_or(DatasetError::MissingArgumentError {
                    idx,
                    text: line.to_string(),
                })?
                .to_string();

            if itr.next().is_some() {
                Err(DatasetError::TooManyArgumentsError {
                    idx,
                    text: line.to_string(),
                })?
            }

            if !data.contains_key(&g) {
                data.insert(g.clone(), Vec::new());
            }
            data.get_mut(&g).unwrap().push([x, y]);
        }
        Ok(Self { data })
    }
}

#[derive(Default)]
pub struct DatasetAssetLoader;

impl AssetLoader for DatasetAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = Dataset::from_buffer(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

#[derive(Debug, Error)]
pub enum DatasetError {
    #[error("Missing argument on line {idx}: {text:?}")]
    MissingArgumentError { idx: usize, text: String },
    #[error("Invalid File Format")]
    InvalidFormatError {
        #[from]
        source: std::str::Utf8Error,
    },
    #[error("Could not parse float")]
    ParsingError {
        #[from]
        source: std::num::ParseFloatError,
    },
    #[error("Too many arguments on line {idx}: {text:?}")]
    TooManyArgumentsError { idx: usize, text: String },
}
