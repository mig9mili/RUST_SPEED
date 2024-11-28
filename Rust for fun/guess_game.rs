extern crate rand;


use std::io; // Using the std::io  library provides you with a number of useful io -related features, including the functionality to accept user input.
use rand::Rng;

fn  main(){

    let secret_number =  rand::thread_rng().gen_range(1,100);
    println!(" Guess the number : ");

    println!("please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you guessed {}", guess);
}