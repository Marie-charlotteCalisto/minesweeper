use rand::distributions::{Distribution, Uniform};
use ansi_term::Colour;

use ggez::GameResult;

use crate::tile;
use crate::default;

pub struct Board {
    board : Vec<tile::Tile>,
    width : usize,
    height : usize,
    tiles_completed : usize,
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

    pub fn is_completed(&self) -> bool{
        self.tiles_completed == self.width * self.height
    }

    pub fn get_tile_completed(&self) -> usize{
        self.tiles_completed
    }

    pub fn check_index_for_rec(&self, row : i32, col : i32) -> bool{
        if row < 0 || col < 0
            || row >= self.height as i32 || col >= self.width as i32{
                false
            }else if self.get(row as usize, col as usize).is_bomb(){
                false
             }else{
                true
            }
    }

    pub fn tile_set_discovered(&mut self, row : usize, col : usize){

        let tile_ = self.get(row, col);
        if tile_.is_flagged() || tile_.is_discovered(){
            return;
        }
        self.tiles_completed += 1;
        let tile = self.get_mut(row, col);
        tile.set_discovered();

        if tile.is_empty(){
            '_row: for offset_row in -1..=1 {
                'col: for offset_col in -1..=1{

                    if offset_row == 0 && offset_row == offset_col{
                        continue 'col;
                    }

                    let new_row = row as i32 + offset_row;
                    let new_col = col as i32 + offset_col;
                    if self.check_index_for_rec(new_row, new_col){
                        self.tile_set_discovered(new_row as usize, new_col as usize);
                    }
                }
            }
        }

    }

    pub fn tile_set_flagged(&mut self, row : usize, col : usize){
        let tile = self.get_mut(row, col);
        if tile.is_discovered() == false {
            if tile.is_flagged() {
                tile.set_flag(false);
                self.tiles_completed -= 1;
            }else {
                tile.set_flag(true);
                self.tiles_completed += 1;
            }
        }
    }

    pub fn new() -> Self {
        let board = vec![tile::Tile::default(); default::BOARD_SIZE.0 * default::BOARD_SIZE.1];

        Board{
            board,
            height : default::BOARD_SIZE.1,
            width : default::BOARD_SIZE.0,
            tiles_completed : 0,
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

