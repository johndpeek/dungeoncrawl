use crate::prelude::*;

pub struct DungeonTheme {}
pub struct ForestTheme{}

impl MapTheme for DungeonTheme {
    fn title_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#')
        }
    }
}

impl MapTheme for ForestTheme {
    fn title_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"')
        }
    }
}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl ForestTheme{
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}