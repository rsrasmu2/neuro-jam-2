use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use tilemap::TilemapPlugin;

use crate::asset_tracking::LoadResource;

pub mod tile;
pub mod tilemap;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<TileAssets>();
        app.add_plugins((TilemapPlugin,));
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct TileAssets {
    #[dependency]
    tile: Handle<Image>,
}

impl TileAssets {
    const PATH_TILE: &str = "sprites/tile.png";
}

impl FromWorld for TileAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            tile: assets.load_with_settings(
                TileAssets::PATH_TILE,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
