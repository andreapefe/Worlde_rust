use std::io;
use std::ptr::null;
use crate::gameplay_all::Mode::{Double, Single, NotStarted};


pub enum Mode{
    NotStarted,
    Single,
    Double,
}

pub struct Gameplay{
    score : u16,
    number_games : u8,
    gamemode : Mode,
}

impl Gameplay {
    //Constructor by default (no other values)
    pub fn new() -> Gameplay{
        Gameplay{
            score : 0,
            number_games : 0,
            gamemode : Mode::NotStarted,
        }
    }

    //Creates the begining banner and gets the gamemode
    pub fn begin(&mut self){
        println!("|*********Welcome to Wordle Game*********|");
        println!("|                                        |");
        println!("| >Type 1 : Single player Game           |");
        println!("| >Type 2 : 1 vs 1 Game                  |");
        println!("| >Type 3 : Quit                         |");
        println!("|                                        |");
        println!("|****************************************|");

        //gets the mode from the user
        let mut correct: bool = false;

        while !correct{
            let mut choice = String::new();
            io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Please type a number!");

            if choice == 1 {
                self.gamemode = Single;
                correct = true
             } else if choice ==2 {
                self.gamemode = Double;
                correct = true
            } else if choice == 3 {
                println!("Quitting...");
                correct = true;
            } else {
                println!("Type 1 or 2");
            }
        }
        match self.gamemode {
            Single => println!("You have chosen  Single player"),
            Double => println!("You have chosen 1 vs 1"),
            NotStarted => ()
        }
    }

    pub fn get_mode(self) -> Mode {
        return self.gamemode;
    }




}

