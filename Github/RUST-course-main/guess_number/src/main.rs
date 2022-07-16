use std::{io, cmp::Ordering};

fn main()
{
    println!("Guess the number");
    println!("Enter your number");
    let secret_number = 10;
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
    .expect("Try a number");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too low"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("You win")
    }
}