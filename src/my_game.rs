use ggez::graphics::{self, Rect};
use cgmath::Point2;
use ggez::{GameResult, Context};

use crate::board;
use crate::tile;
use crate::config_board;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State{
    WON,
    LOST,
    ONGOING,
    REPLAY,
    NOTSTARTED,
}


pub struct MyGame {
    pub board : board::Board,

    pub state : State,
    pub spritebatch : graphics::spritebatch::SpriteBatch,
    pub spritebatch_menu : graphics::spritebatch::SpriteBatch,
}

impl MyGame {

    pub fn new(ctx : &mut Context) -> GameResult<Self> {
        let textures = graphics::Image::new(ctx, "/textures.png")?;
        let textures_menu = graphics::Image::new(ctx, "/textures_menu.png")?;
        let new_board = board::Board::new_randomized()?;

        Ok(
            MyGame {
                board : new_board,
                state : State::NOTSTARTED,
                spritebatch : graphics::spritebatch::SpriteBatch::new(textures),
                spritebatch_menu : graphics::spritebatch::SpriteBatch::new(textures_menu),
            }
          )
    }

    pub fn get_tile_size(&self) -> (usize, usize){
        self.board.get_tile_size()
    }

    pub fn get_width(&self) -> usize{
        self.board.get_width()
    }

    pub fn get_height(&self) -> usize{
        self.board.get_height()
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

    pub fn set_not_started(&mut self){
        self.state = State::NOTSTARTED;
    }

    pub fn is_not_started(&mut self) -> bool{
        self.state == State::NOTSTARTED
    }

    pub fn set_difficulty(&mut self, diff : config_board::Difficulty) {
        self.board.set_difficulty(diff);
    }

    pub fn get_difficulty(&self, row : usize, col : usize) -> config_board::Difficulty {
        if row >= (self.get_height() / 2) - 4
            && row <= (self.get_height() / 2) - 3
                && col >= (self.get_width() / 2) - 2
                && col <= (self.get_width() / 2){
                    config_board::Difficulty::EASY
                }else if row >= (self.get_height() / 2) - 4
                    && row <= (self.get_height() / 2) - 3
                        && col >= (self.get_width() / 2) - 5
                        && col <= (self.get_width() / 2) - 3{
                            config_board::Difficulty::MEDIUM
                        }else{
                            config_board::Difficulty::HARD
                        }


    }
    pub fn set_game_won(&mut self) {
        self.state = State::WON;
    }

    pub fn is_game_won(&self) -> bool{
        self.state == State::WON
    }

    pub fn display_menu(&mut self){
        for i in 0..self.board.get_width() {
            for j in 0..self.board.get_height() {

                let tile = self.board.get(j, i);
                let p =
                    graphics::DrawParam::new()
                    .src(Rect::new(4. / 8., 0. / 8., 1. / 8., 1. / 8.))
                    .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32));
                self.spritebatch_menu.add(p);
            }
        }

        let p_easy =  graphics::DrawParam::new()
            .src(Rect::new(0. / 8., 0. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) - 4)* self.get_tile_size().1) as f32));

        let p_medium =  graphics::DrawParam::new()
            .src(Rect::new(0. / 8., 2. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) - 1)* self.get_tile_size().1) as f32));

        let p_hard =  graphics::DrawParam::new()
            .src(Rect::new(0. / 8., 4. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) + 2)* self.get_tile_size().1) as f32));



        self.spritebatch_menu.add(p_easy);
        self.spritebatch_menu.add(p_medium);
        self.spritebatch_menu.add(p_hard);
    }

    pub fn display_game_over(&mut self) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {

                let tile = self.board.get(j, i);
                let p =
                    match tile.get_tile_type() {
                        tile::TileType::Hint(hint) => {
                            graphics::DrawParam::new()
                                .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        }
                        _ => {
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 0. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        },

                    };

                self.spritebatch.add(p);
            }
        }

        let p =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 0. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) - 2)* self.get_tile_size().1) as f32));

        let p_replay =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 4. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) + 1)* self.get_tile_size().1) as f32));


        self.spritebatch.add(p);
        self.spritebatch.add(p_replay);
    }
    pub fn display_ongoing_game(&mut self) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {

                let tile = self.board.get(j, i);
                let p =
                    if !tile.is_discovered(){
                        if tile.is_flagged(){
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 1. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        }else{
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 2. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        }
                    }

                    else {
                        match tile.get_tile_type() {
                            tile::TileType::Hint(hint) => {
                                graphics::DrawParam::new()
                                    .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                                    .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                            }
                            _ => {
                                graphics::DrawParam::new()
                                    .src(Rect::new(2. / 8., 0. / 8., 1. / 8., 1. / 8.))
                                    .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                            },
                        }

                    };

                self.spritebatch.add(p);
            }
        }
    }
    pub fn display_game_won(&mut self) {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {

                let tile = self.board.get(j, i);
                let p =
                    match tile.get_tile_type() {
                        tile::TileType::Hint(hint) => {
                            graphics::DrawParam::new()
                                .src(Rect::new(((hint % 2) as f32)/ 8., ((hint / 2) as f32) / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        }
                        _ => {
                            graphics::DrawParam::new()
                                .src(Rect::new(2. / 8., 1. / 8., 1. / 8., 1. / 8.))
                                .dest(Point2::new((i * self.get_tile_size().0) as f32, (j * self.get_tile_size().1) as f32))
                        },
                    };


                self.spritebatch.add(p);
            }
        }

        let p =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 2. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) - 2) * self.get_tile_size().1) as f32));

        let p_replay =  graphics::DrawParam::new()
            .src(Rect::new(3. / 8., 4. / 8., 4. / 8., 2. / 8.))
            .dest(Point2::new((((self.get_width() / 2) - 2) * self.get_tile_size().0) as f32, (((self.get_height() / 2) + 1) * self.get_tile_size().1) as f32));


        self.spritebatch.add(p);
        self.spritebatch.add(p_replay);

    }


}
