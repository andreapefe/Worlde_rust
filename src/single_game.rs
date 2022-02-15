use std::io;
use crate::single_game::mode::{Double, Single, NotStarted};


pub enum mode{
    NotStarted,
    Single,
    Double,
}

pub struct gameplay{
    score : u16,
    number_games : u8,
    gamemode : mode,
}

impl gameplay {
    //Constructor by default (no other values)
    pub fn new() -> gameplay{
        gameplay{
            score : 0,
            number_games : 0,
            gamemode : mode::NotStarted,
        }
    }

    //Creates the begining banner and gets the gamemode
    pub fn begin(&mut self){
        println!("|*********Welcome to Wordle Game*********|");
        println!("|                                        |");
        println!("| >Type 1 : Single player Game           |");
        println!("| >Type 2 : 1 vs 1 Game                  |");
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
            } else {
                println!("Type 1 or 2");
            }
        }
        print!("You have chosen ");
        match self.gamemode {
            Single => println!("Single player"),
            Double => println!("1 vs 1"),
            NotStarted => println!("No Mode was selected - Error"),
        }
    }




}

