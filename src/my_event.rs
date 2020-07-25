use ggez::{graphics, event, Context, GameResult};
use ggez::event::MouseButton;

use crate::my_game::MyGame;
use crate::default;
use crate::board;

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        if self.is_not_started(){
            self.display_menu();

            graphics::draw(ctx, &self.spritebatch_menu, graphics::DrawParam::new())?;

            self.spritebatch_menu.clear();

            graphics::present(ctx)?;

            return Ok(());

        }else if self.is_replay() {
            self.board = board::Board::new_randomized()?;
            self.set_ongoing();
        }else if self.is_game_over(){
            self.display_game_over();
        }else if self.is_game_won(){
            self.display_game_won();
        }
        else{
            self.display_ongoing_game();
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


        if self.is_not_started() {
            if button == MouseButton::Left{
                let difficulty = self.get_difficulty(i, j);
                self.set_difficulty(difficulty);
                self.set_ongoing();
            }
        }
        else {
            match button {
                MouseButton::Right => self.board.tile_set_flagged(i, j),
                MouseButton::Left => {
                    if self.is_game_over() || self.is_game_won(){
                        self.set_replay();
                    }else if self.board.get(i, j).is_bomb(){
                        self.set_game_over();
                    }
                    self.board.tile_set_discovered(i, j);
                },
                _ => print!("Not handled"),
            };

            if !self.is_replay() && self.board.is_completed(){
                self.set_game_won();
            }
        }
    }
}
