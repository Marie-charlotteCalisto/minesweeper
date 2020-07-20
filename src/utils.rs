pub fn get_width(nb_bomb : i32) -> usize
{
    let width = nb_bomb as usize;
    width * 30 / 100
}

pub fn get_height(nb_bomb : i32) -> usize
{
    let width = nb_bomb as usize;
    width * 16 / 100
}
