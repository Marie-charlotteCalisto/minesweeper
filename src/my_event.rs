use ggez::{event, Context, GameResult};
use ggez::graphics::{self, Rect};
use ggez::nalgebra::{Point2};


use super::DEFAULT_BOARD_SIZE;
use super::DEFAULT_TILE_SIZE;

use crate::my_game::{MyGame, Tile};

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for i in 0..DEFAULT_BOARD_SIZE.1 {
            for j in 0..DEFAULT_BOARD_SIZE.0 {
                let i_float = i as f32;
                let j_float = j as f32;

                let p = match self.board[i * DEFAULT_BOARD_SIZE.0 + j] {
                    Tile::Bomb => {
                        graphics::DrawParam::new()
                            .src(Rect::new(2. / 8., 0. / 8., 1. / 8., 1. / 8.))
                            .dest(Point2::new(j_float * (DEFAULT_TILE_SIZE.0 as f32), i_float * (DEFAULT_TILE_SIZE.1 as f32)))
                    },
                    Tile::Hint(hint) => {
                        graphics::DrawParam::new()
                            .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                            .dest(Point2::new(j_float * (DEFAULT_TILE_SIZE.0 as f32), i_float * (DEFAULT_TILE_SIZE.1 as f32)))
                    }
                };

                self.spritebatch.add(p);
            }
        }

        graphics::draw(ctx, &self.spritebatch, graphics::DrawParam::new())?;

        self.spritebatch.clear();

        graphics::present(ctx)?;

        Ok(())
    }
}
