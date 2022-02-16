use crate::single::game_single;
use crate::single_game::gameplay;

mod window;
mod single_game;
mod single;

fn main(){
    let mut game1 : gameplay = gameplay::new();
    let mut game : game_single = game_single::new();
    game.get_word();
    game.run_game();
    //game1.begin();
    //match main_window();
    //window::main_window();


}


