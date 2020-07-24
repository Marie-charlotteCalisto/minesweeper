use ggez::{event, Context, GameResult};
use ggez::graphics::{self, Rect};
use ggez::nalgebra::{Point2};
use ggez::event::MouseButton;

use crate::my_game::{MyGame};
use crate::default;
use crate::tile;

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for i in 0..self.board.get_width() {
            for j in 0..self.board.get_height() {
                // let i_float = i as f32;
                // let j_float = j as f32;

                let tile = self.board.get(j, i);
                let p =
                    if !tile.is_discovered(){
                        if tile.is_flagged(){
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 1. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                        }else{
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 2. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                        }
                    }

                    else {
                        match tile.get_tile_type() {
                            tile::TileType::Hint(hint) => {
                                graphics::DrawParam::new()
                                    .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                                    .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                            }
                            _ => {
                                graphics::DrawParam::new()
                                    .src(Rect::new(2. / 8., 0. / 8., 1. / 8., 1. / 8.))
                                    .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                            },
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


    fn mouse_button_down_event(&mut self, _ctx : &mut Context, button : event::MouseButton, x : f32, y : f32)
    {

        let i = (y / (default::TILE_SIZE.1 as f32)) as usize;
        let j = (x / (default::TILE_SIZE.0 as f32)) as usize;


        match button {
            MouseButton::Right => self.board.tile_has_been_flagged(i, j),
            MouseButton::Left => self.board.tile_has_been_discovered(i, j),
            _ => print!("Not handled"),
        };
    }
}
