use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop{
        println!("Please enter a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is smaller!"),
            Ordering::Greater => println!("Your guess is higher"),
            Ordering::Equal => {
                println!("Your win!!!");
                break;
            }

        }
    }
}
