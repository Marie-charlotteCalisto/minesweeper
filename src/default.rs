pub const BOMB_NUMBER : u32 = 100;
pub const BOARD_SIZE : (usize, usize) = (30, 16);
pub const TILE_SIZE : (usize, usize) = (32, 32);
pub const SCREEN_SIZE : (f32, f32) = (
    (BOARD_SIZE.0 as f32)* (TILE_SIZE.0 as f32),
    (BOARD_SIZE.1 as f32)* (TILE_SIZE.1 as f32)
    );


