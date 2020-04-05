use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
    let mut guesses = 6;
    let mut guessed_letters = Vec::new();

    while guesses > 0 {
        let mut guess = String::new();
        let mut word_guess = String::new();
        for char in word.chars() {
            if guessed_letters.contains(&char.to_string()) {
                word_guess.push_str(&char.to_string());
            } else {
                word_guess.push_str("_");
            }
        }
        if !word_guess.contains("_") {
            println!("You win!");
            break;
        }
        println!("{}", word_guess);
        stdin().read_line(&mut guess).expect("Could not read line");
        if !word.contains(&guess.trim().to_string()) {
            guesses -= 1;
        }
        guessed_letters.push(guess.trim().to_string());
        println!("You've guessed: {:?}", guessed_letters);
        println!("Guesses left: {}", guesses);
    }
}
