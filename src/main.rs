use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game");

    let secret_no = rand::thread_rng().gen_range(0, 100);

    loop {
        println!("Please enter a number:");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Enter valid integer");

        let guess: u32 = guess.trim().parse().expect("Enter a valid number");

        match guess.cmp(&secret_no) {
            Ordering::Less => println!("Too Low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
