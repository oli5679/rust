use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let sec_num = rand::thread_rng().gen_range(1,101);

    println!("Secret number {}",sec_num);

    loop {
        let mut guess = String::new();
        std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 =match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }

}
