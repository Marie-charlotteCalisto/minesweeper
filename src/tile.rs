
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileType {
    Bomb,
    Hint(u32),
}

impl Default for TileType {
    fn default() -> Self{ TileType::Hint(0) }
}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Tile {
    tile : TileType,
    is_discovered : bool,
    is_flagged : bool,
}


impl Tile {

    pub fn is_bomb(&self) -> bool{
        self.tile == TileType::Bomb
    }

    pub fn is_discovered(&self) -> bool{
        self.is_discovered
    }

    pub fn is_flagged(&self) -> bool{
        self.is_flagged
    }

    pub fn set_flag(&mut self, flagged : bool){
        self.is_flagged = flagged;
    }

    pub fn set_discovered(&mut self) {
        self.is_discovered = true;
    }

    pub fn new_bomb() -> Self{

        Tile {
            tile : TileType::Bomb,
            is_discovered : false,
            is_flagged : false,
        }
    }

    pub fn get_hint(&self) -> u32 {
        match self.tile{
            TileType::Hint(hint) => hint,
            _ => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.get_hint() == 0
    }

    pub fn get_tile_type(&self) -> TileType{
        self.tile
    }

    pub fn add_hint(&mut self) {
        self.tile = match self.tile {
            TileType::Bomb => TileType::Bomb,
            TileType::Hint(hint) => TileType::Hint(hint + 1),
        };
    }

}


