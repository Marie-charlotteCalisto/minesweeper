use std::path;

use ggez::{ContextBuilder, GameResult};
use ggez::event;

mod my_game;

const DEFAULT_BOMB_NUMBER : u32 = 100;
const DEFAULT_BOARD_SIZE : (usize, usize) = (30, 16);
const DEFAULT_TILE_SIZE : (usize, usize) = (32, 32);
const DEFAULT_SCREEN_SIZE : (f32, f32) = (
    (DEFAULT_BOARD_SIZE.0 as f32)* (DEFAULT_TILE_SIZE.0 as f32),
    (DEFAULT_BOARD_SIZE.1 as f32)* (DEFAULT_TILE_SIZE.1 as f32)
    );



fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let (ctx, events_loop) = ContextBuilder::new("minesweeper", "macha")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("minesweeper").vsync(false))
        .window_mode(ggez::conf::WindowMode::default().dimensions(DEFAULT_SCREEN_SIZE.0, DEFAULT_SCREEN_SIZE.1))
        .build()?;

    let mut my_game = MyGame::new(&mut ctx)?;

    match event::run(&mut ctx, &mut events_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured : {}", e)
    }

    Ok(())
}
