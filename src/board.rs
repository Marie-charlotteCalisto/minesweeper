use rand::distributions::{Distribution, Uniform};

pub struct Board {
    board : Vec<usize>,
    height : usize,
    width : usize,
}


impl Board
{
    pub fn create_board(mut nb_bomb : i32, h : usize, w : usize) -> Board
    {
        let size = h*w;
        let mut new_board : Vec<usize> = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..size);

        for _ in 0..size{
            new_board.push(0);
        }
        for _ in 0..h {
            for _ in 0..w {
                if nb_bomb > 0
                {
                    let mut pos = range.sample(&mut rng);
                    while new_board[pos] == 1 {
                        pos = range.sample(&mut rng);
                    }
                    new_board[pos] = 1;
                    nb_bomb -= 1;
                }
            }
        }
        Board {
            board : new_board,
            height : h,
            width : w,
        }
    }

    pub fn print_board(self)
    {

        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.board[i*self.width + j]);
            }
            println!();
        }
    }
}
