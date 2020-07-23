use rand::distributions::{Distribution, Uniform};

use ggez::{graphics, GameResult, Context};

use super::DEFAULT_BOMB_NUMBER;
use super::DEFAULT_BOARD_SIZE;

#[derive(Eq, PartialEq)]
pub enum Tile {
    Bomb,
    Hint(u32),
}


pub struct MyGame {
    nb_bomb : u32,
    pub board : Vec<Tile>,

    pub spritebatch : graphics::spritebatch::SpriteBatch,
}

impl MyGame {

    pub fn new(ctx : &mut Context) -> GameResult<MyGame> {
        let textures = graphics::Image::new(ctx, "/textures.png")?;
        let new_board = init_board();

        Ok(
            MyGame {
                nb_bomb : DEFAULT_BOMB_NUMBER,
                board : new_board,

                spritebatch : graphics::spritebatch::SpriteBatch::new(textures),
            }
          )
    }
}

fn init_board() -> Vec<Tile> {

    let mut board = Vec::<Tile>::with_capacity(DEFAULT_BOARD_SIZE.0 * DEFAULT_BOARD_SIZE.1);

    let mut rng = rand::thread_rng();
    let range_width = Uniform::from(0..DEFAULT_BOARD_SIZE.0);
    let range_height = Uniform::from(0..DEFAULT_BOARD_SIZE.1);

    for _ in 0..(DEFAULT_BOARD_SIZE.0 * DEFAULT_BOARD_SIZE.1){
        board.push(Tile::Hint(0));
    }


    for _ in 0..DEFAULT_BOMB_NUMBER{
        let mut pos_rand_width = range_width.sample(&mut rng);
        let mut pos_rand_height = range_height.sample(&mut rng);

        while board[pos_rand_height * DEFAULT_BOARD_SIZE.0 + pos_rand_width] == Tile::Bomb {
            pos_rand_width = range_width.sample(&mut rng);
            pos_rand_height = range_height.sample(&mut rng);
        }

        board[pos_rand_height * DEFAULT_BOARD_SIZE.0 + pos_rand_height] = Tile::Bomb;
        let pos_width = pos_rand_width as i32;
        let pos_height = pos_rand_height as i32;
        add_hint(&mut board, pos_width, pos_height + 1);
        add_hint(&mut board, pos_width, pos_height - 1);
        add_hint(&mut board, pos_width - 1, pos_height);
        add_hint(&mut board, pos_width - 1, pos_height + 1);
        add_hint(&mut board, pos_width - 1, pos_height - 1);
        add_hint(&mut board, pos_width + 1, pos_height);
        add_hint(&mut board, pos_width + 1, pos_height + 1);
        add_hint(&mut board, pos_width + 1, pos_height - 1);

    };

    board
}

fn add_hint(board : &mut Vec<Tile>, i : i32, j : i32){
    if i < 0 || j < 0 {
        return
    }

    let index_width = i as usize;
    let index_height = j as usize;
    if index_width >= DEFAULT_BOARD_SIZE.0 || index_height >= DEFAULT_BOARD_SIZE.1{
        return
    }
    let pos = index_height * DEFAULT_BOARD_SIZE.0 + index_width;
    board[pos] =
        match board[pos] {
            Tile::Bomb => Tile::Bomb,
            Tile::Hint(hint) => Tile::Hint(hint + 1),
        };
}


