use crate::single::GameSingle;
use crate::gameplay_all::{Gameplay, Mode};

mod window;
mod gameplay_all;
mod single;



fn main(){
    let mut game1 : Gameplay = Gameplay::new();
    //start the game
    game1.begin();
    match game1.get_mode()
    {
        //Single gmae
        Mode::Single => {
            let mut game : GameSingle = GameSingle::new();
            game.get_word();
            game.run_game();
        },

        //1v1 game, by turns each selects a word for the other to guess
        Mode::Double => {println!("Not implemented")},

        //error in .begin
        _ => {println!("Goodbye!")}
    }



    //game1.begin();
    //match main_window();
    //window::main_window();


}


