use crate::*;

#[derive(Clone, PartialEq)]
pub enum Tile {
    Water = 203,
    Transparent = 16,
    GreenFloor = 23,
}

impl Tile {
    fn is_passable(&self) -> bool {
        match self {
            Tile::Transparent => true,
            Tile::GreenFloor => true,
            Tile::Water => false,
        }
    }
}

pub struct TileType {
    sprite: usize,
    passable: bool,

}

