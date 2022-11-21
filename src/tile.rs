use crate::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Water = 203,
    Transparent = 16,
    Floor = 23,
}

impl Tile {
    fn is_passable(&self) -> bool {
        match self {
            Tile::Transparent => true,
            Tile::Floor => true,
            Tile::Water => false,
        }
    }
}
