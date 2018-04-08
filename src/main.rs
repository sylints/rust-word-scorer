use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading file");

    //println!("Enter a word to receive a score: ");
    //let mut user_word = String::new();
    //io::stdin()
    //    .read_line(&mut user_word)
    //    .expect("Failed to read user word");

    let score = get_score(&contents).expect("You entered a character that is not a-z");

    println!("Total Points: {}", score);
}

fn get_score(word: &str) -> Result<u32, char> {
    let mut total_points: u32 = 0;
    for l in word.chars() {
        match l {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 's' | 't' | 'r' => total_points += 1,
            'd' | 'g' => total_points += 2,
            'b' | 'c' | 'm' | 'p' => total_points += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => total_points += 4,
            'k' => total_points += 5,
            'j' | 'x' => total_points += 8,
            'q' | 'z' => total_points += 10,
            _ => total_points += 0,
            //_ => return Err(l),
        }
    }
    Ok(total_points)
}
