use std::{cmp::Ordering, io };

use rand::random_range;


fn main(){


    let secret_number = random_range(1..=100);

    loop{
        println!("enter num: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("reading error");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("ure guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("u win");
                break;
            },
            Ordering::Greater => println!("big"),
            Ordering::Less => println!("small"),
        }
    }
}   

