use crate::single_game::gameplay;

mod window;
mod single_game;
mod single;

fn main(){
    let mut game1 : gameplay = gameplay::new();
    game1.begin();
    //match main_window();
    //window::main_window();
}


