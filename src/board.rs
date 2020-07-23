use rand::distributions::{Distribution, Uniform};
use ansi_term::Colour;

use ggez::GameResult;

use crate::tile;
use crate::default;

pub struct Board {
    board : Vec<tile::Tile>,
    width : usize,
    height : usize,
}

impl Board {
    pub fn get(&self, row : usize, col: usize) -> &tile::Tile{
        &self.board[row * self.width + col]
    }

    pub fn get_mut(&mut self, row : usize, col: usize) -> &mut tile::Tile{
        &mut self.board[row * self.width + col]
    }




    pub fn set(&mut self, row : usize, col : usize, tile : tile::Tile) {
        self.board[row * self.width + col] = tile;
    }

    pub fn get_width(&self) -> usize{
        self.width
    }

    pub fn get_height(&self) -> usize{
        self.height
    }


    pub fn tile_has_been_discovered(&mut self, row : usize, col : usize){
        let mut tile = self.get_mut(row, col);
        tile.has_been_discovered();

    }

    pub fn tile_has_been_flagged(&mut self, row : usize, col : usize){
        let mut tile = self.get_mut(row, col);
        if tile.is_discovered() == false {
            println!("toto");
            tile.set_flag(!tile.is_flagged());
        }
    }

    pub fn new() -> Self {
        let board = vec![tile::Tile::default(); default::BOARD_SIZE.0 * default::BOARD_SIZE.1];

        Board{
            board,
            height : default::BOARD_SIZE.1,
            width : default::BOARD_SIZE.0,
        }
    }

    pub fn new_randomized() -> GameResult<Self> {
        let mut board = Self::new();

        let mut rng = rand::thread_rng();
        let range_col = Uniform::from(0..board.width);
        let range_row = Uniform::from(0..board.height);



        for _ in 0..default::BOMB_NUMBER{
            let mut pos_rand_row = range_row.sample(&mut rng);
            let mut pos_rand_col = range_col.sample(&mut rng);

            while board.get(pos_rand_row, pos_rand_col).is_bomb(){
                pos_rand_row = range_row.sample(&mut rng);
                pos_rand_col = range_col.sample(&mut rng);
            }

            board.set(pos_rand_row, pos_rand_col, tile::Tile::new_bomb());

            board.add_all_hints(pos_rand_row, pos_rand_col);
        };

        board.debug_print_board();
        Ok (board)
    }


    fn add_all_hints(&mut self, i : usize, j : usize) {
        let pos_row = i as i32;
        let pos_col = j as i32;

        self.add_hint(pos_row, pos_col + 1);
        self.add_hint(pos_row, pos_col - 1);
        self.add_hint(pos_row - 1, pos_col);
        self.add_hint(pos_row - 1, pos_col + 1);
        self.add_hint(pos_row - 1, pos_col - 1);
        self.add_hint(pos_row + 1, pos_col);
        self.add_hint(pos_row + 1, pos_col + 1);
        self.add_hint(pos_row + 1, pos_col - 1);
    }

    pub fn add_hint(&mut self, i : i32, j : i32){
        if i < 0 || j < 0 {
            return
        }

        let row = i as usize;
        let col = j as usize;
        if row >= self.height || col >= self.width {
            return
        }
        self.get_mut(row, col).add_hint();
    }

    pub fn debug_print_board(&self)
    {
        for i in 0..self.height {
            for j in 0..self.width {
                let val = self.get(i, j);
                let v = val.get_hint();
                let color = (v * 20) as u8;
                let background = Colour::Fixed(240);
                if val.is_bomb(){
                    print!("{}", Colour::Fixed(124).on(background).paint(" X"));
                }
                else{
                    print!("{}", Colour::Fixed(color).on(background).paint(format!(" {}", v.to_string())));
                }

            }
            println!();
        }
    }

}

