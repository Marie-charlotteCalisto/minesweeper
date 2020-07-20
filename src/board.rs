use rand::distributions::{Distribution, Uniform};
use ansi_term::Colour;

pub struct Board {
    board : Vec<i32>,
    height : usize,
    width : usize,
}


impl Board
{
    pub fn create(mut nb_bomb : i32) -> Board
    {
        let h = Board::get_height(nb_bomb);
        let w = Board::get_width(nb_bomb);
        println!("the board is of size : {:?} {:?}", h, w);
        let size = h*w;
        let mut board = Board {
            board : Vec::with_capacity(size),
            height : h,
            width : w,
        };

        let mut rng = rand::thread_rng();
        let range_w = Uniform::from(0..w);
        let range_h = Uniform::from(0..h);

        for _ in 0..size{
            board.board.push(0);
        }
        while nb_bomb > 0
        {
            let mut pos_rand_w = range_w.sample(&mut rng);
            let mut pos_rand_h = range_h.sample(&mut rng);
            while board.board[pos_rand_h* w + pos_rand_w] == -1 {
                pos_rand_w = range_w.sample(&mut rng);
                pos_rand_h = range_h.sample(&mut rng);
            }
            board.board[pos_rand_h*w + pos_rand_w] = -1;
            let pos_w = pos_rand_w as i32;
            let pos_h = pos_rand_h as i32;
            board.add_one(pos_h, pos_w - 1);
            board.add_one(pos_h, pos_w + 1);
            board.add_one(pos_h - 1, pos_w);
            board.add_one(pos_h + 1, pos_w);
            board.add_one(pos_h + 1, pos_w + 1);
            board.add_one(pos_h + 1, pos_w - 1);
            board.add_one(pos_h - 1, pos_w - 1);
            board.add_one(pos_h - 1, pos_w + 1);
            nb_bomb -= 1;
        }
        board
    }


    fn add_one(&mut self, i : i32, j : i32)
    {
        if i < 0 || j < 0 {
            return
        }
        let index_i = i as usize;
        let index_j = j as usize;
        if index_i >= self.height || index_j >= self.width
         || self.board[index_i * self.width + index_j] == -1 {
            return
        }
        self.board[index_i * self.width + index_j] += 1;
    }

    fn get_width(nb_bomb : i32) -> usize
    {
        let width = nb_bomb as usize;
        width * 30 / 100
    }

    fn get_height(nb_bomb : i32) -> usize
    {
        let width = nb_bomb as usize;
        width * 16 / 100
    }

    pub fn print_board(self)
    {

        for i in 0..self.height {
            for j in 0..self.width {
                let val = self.board[i * self.width + j];
                match val{
                    -1 => print!(" {}", Colour::Red.paint("X")),
                    1 => print!(" {}", Colour::Blue.paint(val.to_string())),
                    2 => print!(" {}", Colour::Yellow.paint(val.to_string())),
                    3 => print!(" {}", Colour::Green.paint(val.to_string())),
                    4 => print!(" {}", Colour::Cyan.paint(val.to_string())),
                    v => print!(" {}", v),
                }

            }
            println!();
        }
    }
}
