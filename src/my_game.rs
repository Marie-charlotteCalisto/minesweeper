
use ggez::{graphics, GameResult, Context};

use crate::default;
use crate::board;

pub struct MyGame {
    _nb_bomb : u32,
    pub board : board::Board,

    pub spritebatch : graphics::spritebatch::SpriteBatch,
}

impl MyGame {

    pub fn new(ctx : &mut Context) -> GameResult<Self> {
        let textures = graphics::Image::new(ctx, "/textures.png")?;
        let new_board = board::Board::new_randomized()?;

        Ok(
            MyGame {
                _nb_bomb : default::BOMB_NUMBER,
                board : new_board,

                spritebatch : graphics::spritebatch::SpriteBatch::new(textures),
            }
          )
    }
}
