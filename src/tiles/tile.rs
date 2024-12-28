use bevy::prelude::*;

pub const TILE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

#[derive(Component, Debug)]
#[require(Sprite)]
pub struct Tile {
    pub position: UVec2,
}

impl Tile {
    pub fn world_position(&self) -> Vec2 {
        self.position.as_vec2() * TILE_SIZE
    }
}
