use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::thiserror,
};
use rapid_qoi::{Colors, DecodeError, Qoi};

struct QOIAssetLoader;

#[derive(Debug, thiserror::Error)]
pub enum QOILoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to decode QOI: {0}")]
    Decode(#[from] DecodeError),
    #[error("Bevy doesn't support RGB (without alpha) images; format was {0:?}")]
    NoAlpha(Colors),
}

impl AssetLoader for QOIAssetLoader {
    type Asset = Image;
    type Settings = ();
    type Error = QOILoadError;
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let (header, decoded) = Qoi::decode_alloc(&bytes)?;
            let format = match header.colors {
                Colors::Rgb | Colors::Srgb => Err(QOILoadError::NoAlpha(header.colors)),
                Colors::Rgba | Colors::SrgbLinA => Ok(TextureFormat::Rgba8UnormSrgb),
            }?;
            let image = Image::new(
                Extent3d {
                    width: header.width,
                    height: header.height,
                    ..default()
                },
                TextureDimension::D2,
                decoded,
                format,
            );
            Ok(image)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["qoi"]
    }
}

/// Plugin that registers the QOIAssetLoader
pub struct QOIPlugin;

impl Plugin for QOIPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(QOIAssetLoader);
    }
}
