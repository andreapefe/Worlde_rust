use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;
use std::env;
use std::fs::{copy, File, read_to_string};

use std::io::Error;
use std::path::Path;
use std::ptr::null;
use std::string::String;


pub struct game_single{
    random_word : String,
    number_guesses : u8,
    won : bool,
}

impl game_single {

    pub fn new() -> game_single{
        game_single{
            random_word : String::new(),
            number_guesses : 10,
            won : false,
        }
    }

    pub fn get_word(&mut self){
        //Read file with 5 letter words

        let path = Path::new("05.txt");
        let display = path.display();
        //get file
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        //convert to string
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("Bien lu"),
        }
        //count number of lines
        let mut count = 0;
        for c in s.chars(){
            if c == '\n'{
                count += 1;
            }
        }

        println!("Nombre de lignes: {} ", count);

        //Get random word

        //get random number between 1 and max lines
        let mut number = rand::thread_rng().gen_range(1..count);
        let mut found = false;
        println!("{}", number);
        //get the random word
        for c in s.chars(){
            if (found) & (c != '\n'){
                self.random_word.push(c);
            } else if c == '\n'{
                number -= 1;
            }
            if number == 0{
                found = true;
            } else if number < 0 {
                found = false;
            }
        }
        println!("Word selected!");
        self.random_word.push('\n');
        println!("{}", self.random_word);
    }

    fn find_word(word : &String) -> bool {
        let mut found = false;
        let path = Path::new("05.txt");
        let display = path.display();
        //get file
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        //convert to string
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }
        let mut check = String::new();

        for c in s.chars(){
            check.push(c);
            if check.eq(word){
                found = true;
                break;
            }
            if c == '\n'{
                check.clear();
            }
        }

        return found;
    }

    pub fn run_game(&mut self){
        //loop with enough guesses
        while !self.won{

            println!("Number of remaining guesses : {} ", self.number_guesses);
            println!("Enter Guess : ");

            //Loop to check the guess is 5 letters long and exists
            let mut correct = false;
            let mut guess = String::with_capacity(5);
            while !correct {
                let mut input = String::with_capacity(5);
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line. Enter a 5 letter word");

                if input.chars().count() != 6{
                    println!("Enter a 5 letter word")
                } else if !game_single::find_word(&input){
                    println!("This word does not exist. Try again")
                } else { correct = true; guess = String::from(input);}

            }

            //Check it is the correct word
            if guess.eq(&self.random_word){
                println!("YOU WON!");
                self.won = true;
            } else {
                self.number_guesses -= 1; //One less guess is available
                let mut contain = String::new() ;
                print!("Contains :");
                //check if it contains the letters, only once
                for c in guess.chars(){
                    let mut  double
                    if self.random_word.contains(c) & !(contain.contains(c)){
                        contain.push(c);
                        print!(" {}", c);
                    }
                }

                //Check if they are at the correct place
                let mut random_indices = self.random_word.char_indices();
                let mut guess_indices = guess.char_indices();
                print!("Right place :");
                for i in 0..5{
                    if &random_indices.next() == &guess_indices.next(){
                        print!(" {}", guess.chars().nth(i).unwrap());
                    }
                }
                println!("");

                println!("Try again!")
            }
        }
    }

}