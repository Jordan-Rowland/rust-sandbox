use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let correct_number = rand::thread_rng().gen_range(1, 21);
    let mut turns: u32 = 3;
    println!("Guess the number!");

    loop {
        if turns == 0 {
            println!("\nAll out of turns!");
            break;
        }

        println!("Please input your guess! {} turns", turns);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                continue;
            }
        };

        match guess.cmp(&correct_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        turns -= 1;
    }
}
