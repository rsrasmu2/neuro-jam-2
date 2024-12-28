use bevy::{prelude::*, sprite::Anchor, utils::HashMap};

use super::{
    tile::{Tile, TILE_SIZE},
    TileAssets,
};

pub(super) struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_systems);
    }
}

#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct Tilemap {
    tiles: HashMap<UVec2, Entity>,
}

impl Tilemap {
    fn new(dimensions: UVec2) -> Self {
        Self {
            tiles: HashMap::with_capacity((dimensions.x * dimensions.y) as usize),
        }
    }
}

pub struct SpawnTilemap {
    pub dimensions: UVec2,
}

impl Command for SpawnTilemap {
    fn apply(self, world: &mut World) {
        world.run_system_cached_with(spawn_tilemap, self).unwrap();
    }
}

fn spawn_tilemap(
    In(settings): In<SpawnTilemap>,
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
) {
    let total_size = settings.dimensions.as_vec2() * TILE_SIZE;
    let half_size = total_size / 2.0;
    let tilemap_entity = commands.spawn(Transform::from_translation(-half_size.extend(0.0))).id();

    let mut tilemap = Tilemap::new(settings.dimensions);
    for x in 0..settings.dimensions.x {
        for y in 0..settings.dimensions.y {
            let position = UVec2::new(x, y);
            let tile = Tile { position };
            let tile = commands
                .spawn((
                    Transform::from_translation(tile.world_position().extend(0.0)),
                    tile,
                    Sprite {
                        image: tile_assets.tile.clone(),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                ))
                .set_parent(tilemap_entity)
                .id();
            tilemap.tiles.insert(position, tile);
        }
    }

    commands.entity(tilemap_entity).insert(tilemap);
}

fn register_systems(world: &mut World) {
    world.register_system_cached(spawn_tilemap);
}
