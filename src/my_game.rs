use ggez::nalgebra::{Point2};
use ggez::graphics::{self, Rect};
use ggez::{GameResult, Context};

use crate::default;
use crate::board;
use crate::tile;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State{
    WON,
    LOST,
    ONGOING,
    REPLAY,
}

pub struct MyGame {
    pub board : board::Board,

    pub state : State,
    pub spritebatch : graphics::spritebatch::SpriteBatch,
}

impl MyGame {

    pub fn new(ctx : &mut Context) -> GameResult<Self> {
        let textures = graphics::Image::new(ctx, "/textures.png")?;
        let new_board = board::Board::new_randomized()?;

        Ok(
            MyGame {
                board : new_board,
                state : State::ONGOING,
                spritebatch : graphics::spritebatch::SpriteBatch::new(textures),
            }
          )
    }

    pub fn set_game_over(&mut self){
        self.state = State::LOST;
    }

    pub fn is_game_over(&self) -> bool{
        self.state == State::LOST
    }

    pub fn set_replay(&mut self) {
        self.state = State::REPLAY;
    }

    pub fn is_replay(&self) -> bool{
        self.state == State::REPLAY
    }

    pub fn set_ongoing(&mut self) {
        self.state = State::ONGOING;
    }

    pub fn set_game_won(&mut self) {
        self.state = State::WON;
    }

    pub fn is_game_won(&self) -> bool{
        self.state == State::WON
    }

    pub fn display_game_over(&mut self) {
        for i in 0..self.board.get_width() {
            for j in 0..self.board.get_height() {

                let tile = self.board.get(j, i);
                let p =
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

                    };

                self.spritebatch.add(p);
            }
        }

        let p =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 0. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.board.get_width() / 2) - 2) * default::TILE_SIZE.0) as f32, (((self.board.get_height() / 2) - 2)* default::TILE_SIZE.1) as f32));

        let p_replay =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 4. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.board.get_width() / 2) - 2) * default::TILE_SIZE.0) as f32, (((self.board.get_height() / 2) + 1)* default::TILE_SIZE.1) as f32));


        self.spritebatch.add(p);
        self.spritebatch.add(p_replay);
    }
    pub fn display_ongoing_game(&mut self) {
        for i in 0..self.board.get_width() {
            for j in 0..self.board.get_height() {

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
    }
    pub fn display_game_won(&mut self) {
        for i in 0..self.board.get_width() {
            for j in 0..self.board.get_height() {

                let tile = self.board.get(j, i);
                let p =
                    match tile.get_tile_type() {
                        tile::TileType::Hint(hint) => {
                            graphics::DrawParam::new()
                                .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                        }
                        _ => {
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 1. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * default::TILE_SIZE.0) as f32, (j * default::TILE_SIZE.1) as f32))
                        },
                    };


                self.spritebatch.add(p);
            }
        }

        let p =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 2. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.board.get_width() / 2) - 2) * default::TILE_SIZE.0) as f32, (((self.board.get_height() / 2) - 2)* default::TILE_SIZE.1) as f32));

        let p_replay =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 4. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.board.get_width() / 2) - 2) * default::TILE_SIZE.0) as f32, (((self.board.get_height() / 2) + 1)* default::TILE_SIZE.1) as f32));


        self.spritebatch.add(p);
        self.spritebatch.add(p_replay);

    }


}
