use bevy::prelude::*;

use crate::{screens::Screen, tiles::tilemap::SpawnTilemap};

pub const TILEMAP_SIZE: UVec2 = UVec2::new(11, 11);

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), spawn_tilemap);
    }
}

fn spawn_tilemap(mut commands: Commands) {
    commands.queue(SpawnTilemap {
        dimensions: TILEMAP_SIZE,
    });
}
