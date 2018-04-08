use std::io;

fn main() {
    println!("Enter a word to receive a score: ");
    let mut user_word = String::new();
    io::stdin()
        .read_line(&mut user_word)
        .expect("Failed to read user word");

    let score = get_score(user_word.trim()).expect("You entered a character that is not a-z");

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
            _ => return Err(l),
        }
    }
    Ok(total_points)
}
