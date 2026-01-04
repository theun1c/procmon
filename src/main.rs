use std::{cmp::Ordering, io};

use rand::{ random_range };

fn main(){

    let secret_number= random_range(1..=100);

    println!("enter guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("error while reading");

    let guess: u32 = guess.trim().parse().expect("converting error");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("to small"),
        Ordering::Equal => println!("u win"),
        Ordering::Greater => println!("to big"),
    }
    
}   

