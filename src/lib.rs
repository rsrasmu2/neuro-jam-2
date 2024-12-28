use bevy::{asset::AssetMetaCheck, prelude::*};
use camera::GameCameraPlugin;
use game::GamePlugin;
use screens::ScreensPlugin;
use tiles::TilesPlugin;

pub mod asset_tracking;
mod camera;
mod game;
mod screens;
mod tiles;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Neuro Jam 2".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            asset_tracking::plugin,
            GameCameraPlugin,
            TilesPlugin,
            ScreensPlugin,
            GamePlugin,
        ));
    }
}
