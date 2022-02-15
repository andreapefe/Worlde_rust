use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;
use std::env;
use std::fs::{File, read_to_string};

use std::io::Error;
use std::path::Path;
use std::ptr::null;


pub struct game_single{
    random_word : String,
    number_guesses : u8,
}

impl game_single {

    pub fn new() -> game_single{
        game_single{
            random_word : String::new(),
            number_guesses : 10,
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

        println!("Random word : {} ", self.random_word);
    }

}