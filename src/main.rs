mod utils;
mod board;

fn get_bomb_number() -> i32
{
    let mut bomb = String::new();
    let mut nb_bomb : i32;
    loop
    {
        println!("How many bombs do you want to play with ?");
        if std::io::stdin().read_line(&mut bomb).is_err() {
            println!("error: error read_line" );
            continue;
        }

        bomb.pop();
        nb_bomb = match bomb.parse::<i32>()
        {
            Ok(n) => n,
            Err(_e) => -1,
        };
        if nb_bomb < 0 {
            println!("Not a valid number.");
            bomb = String::new();
            continue;
        }
        break;
    }
    nb_bomb
}


fn main()
{
    let nb_bomb = get_bomb_number();
    let width = utils::get_width(nb_bomb);
    let height = utils::get_height(nb_bomb);
    println!("the board is of size : {:?} {:?}", height, width);

    let board : board::Board = board::Board::create_board(nb_bomb, height, width);

    board::Board::print_board(board);
}
