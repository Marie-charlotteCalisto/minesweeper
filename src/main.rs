mod my_game;
mod my_event;
mod default;
mod tile;
mod board;
mod config_board;

use std::path;

use ggez::{ContextBuilder, GameResult};
use ggez::event;

use my_game::MyGame;


fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let (mut ctx, events_loop) = ContextBuilder::new("minesweeper", "macha")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("minesweeper").vsync(false))
        .window_mode(ggez::conf::WindowMode::default().dimensions(default::SCREEN_SIZE.0, default::SCREEN_SIZE.1))
        .build()?;

    let my_game = MyGame::new(&mut ctx)?;

    event::run(ctx, events_loop, my_game);

    Ok(())
}
