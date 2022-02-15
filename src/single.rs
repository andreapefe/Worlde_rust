use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;
use std::env;
use std::fs::{File, read_to_string};

use std::io::Error;
use std::path::Path;



pub struct game_single{
    random_word : String,
    number_guesses : u8,
}

impl game_single {

    pub fn new() -> game_single{
        game_single{
            random_word : String::with_capacity(5),
            number_guesses : 6,
        }
    }

    pub fn get_word(&mut self){
        //Read file with 5 letter words
        let path = Path::new("05.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        //get random number
        let number = rand::thread_rng().gen_range(1..101);
    }

}