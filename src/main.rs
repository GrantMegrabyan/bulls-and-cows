extern crate rand;

use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Welcome to the Bulls and Cows game!");
    println!("I'm going to think of a 4 digit number, your job is to guess it");

    let secret = gen_number();
    // println!("Pssst! Here is my number: {}", secret);

    println!("Let's start the guessing game!");

    let mut guess;
    loop {
        guess = String::new();
        println!("Make your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        guess = guess.trim().to_owned();

        if guess == "0" {
            break;
        }

        let answer = get_answer(&secret, &guess);

        if answer.0 == secret.len() {
            println!("Correct! You win!!!");
            break;
        }

        println!("You guessed: {}. Bulls: {}, Cows: {}", guess, answer.0, answer.1);
    }
}

fn gen_number() -> String {
    let mut digits: Vec<u8> = (0..10).collect();
    // println!("Digits: {:?}", digits);
    let mut rng = thread_rng();
    digits.shuffle(&mut rng);

    // println!("Shuffled: {:?}", &digits[0..4]);
    let secret = digits[0..4].into_iter().map(|d| d.to_string()).collect::<String>();

    secret
}

fn get_answer(secret: &str, guess: &str) -> (usize, usize) {
    if guess == secret {
        return (guess.len(), 0)
    }

    let guess_chars: Vec<char> = guess.chars().collect();
    let secret_chars: Vec<char> = secret.chars().collect();

    // println!("gc: {:?}, sc: {:?}", guess_chars, secret_chars);

    let mut index: usize = 0;
    let mut bulls: usize = 0;
    let mut cows: usize = 0;
    while index < guess_chars.len() {
        if guess_chars[index] == secret_chars[index] {
            bulls += 1;
        } 
        else if secret_chars.contains(&guess_chars[index]) {
            cows += 1;
        }

        index += 1;
    }

    (bulls, cows)
}