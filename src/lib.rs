use anyhow::anyhow;
use bevy::{
    prelude::*,
    asset::{AssetLoader, LoadedAsset, Error},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use rapid_qoi::{Colors, Qoi};


struct QOIAssetLoader;

impl AssetLoader for QOIAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), Error>> {

        Box::pin(async move {
            let (header, decoded) = Qoi::decode_alloc(&bytes).unwrap();

            load_context.set_default_asset(
                LoadedAsset::new(Image::new(
                    Extent3d {
                        width: header.width,
                        height: header.height,
                        ..Default::default()
                    },
                    TextureDimension::D2,
                    decoded,
                    match header.colors {
                        Colors::Rgb => Err(anyhow!("Rgb not supported.")),
                        Colors::Srgb => Err(anyhow!("Rgb not supported.")),
                        Colors::Rgba => Ok(TextureFormat::Rgba8Unorm),
                        Colors::SrgbLinA => Ok(TextureFormat::Rgba8UnormSrgb),
                    }?
                ))
            );
            Ok(())
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
        app.add_asset_loader(QOIAssetLoader);
    }
}
