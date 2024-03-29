use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        match io::stdin()
            .read_line(&mut guess) {
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to read line!");
                    continue;
                },
            };

        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };  

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        

    }
    

}
