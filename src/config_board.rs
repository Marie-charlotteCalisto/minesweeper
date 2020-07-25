use crate::default;

pub enum Difficulty{
    EASY,
    MEDIUM,
    HARD,
}

pub struct Config{
    bomb_number : u32,
    width : usize,
    height : usize,
    tile_size : (usize, usize),
    screen_size : (f32, f32),
}

impl Config{
    pub fn new() -> Self{
        Config{
            bomb_number : default::BOMB_NUMBER,
            width : default::BOARD_SIZE.0,
            height : default::BOARD_SIZE.1,
            tile_size : default::TILE_SIZE,
            screen_size : default::SCREEN_SIZE,
        }
    }

    pub fn set_config(&mut self, bomb_number : u32, board_size : (usize, usize)){
        self.bomb_number = bomb_number;
        self.width = board_size.0;
        self.height = board_size.1;
        self.screen_size = (
            (self.width as f32) * (self.tile_size.0 as f32),
            (self.height as f32) * (self.tile_size.1 as f32),
            );
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_bomb_number(&self) -> u32 {
        self.bomb_number
    }

    pub fn get_tile_size(&self) -> (usize, usize) {
        self.tile_size
    }

    pub fn get_screen_size(&self) -> (f32, f32) {
        self.screen_size
    }
}
