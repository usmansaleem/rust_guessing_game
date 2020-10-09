use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //TODO: Delete
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //expect will print err msg and crash prog

        // convert guess to unsigned int 32
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less than original"),
            Ordering::Greater => println!("Your guess is greater than original"),
            Ordering::Equal => {
                println!("You win, your guess matches with Computer's");
                break;
            }
        }
    } // loop
}
