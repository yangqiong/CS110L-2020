// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use core::panic;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    let mut guess_word = vec!['-'; secret_word.len()];
    let mut guess_word_chars = String::new();
    let mut guess_count = NUM_INCORRECT_GUESSES;

    loop {
        print_guess_word(&guess_word);
        print_guess_chars(&guess_word_chars);
        println!("You have {} guesses left", guess_count);

        let guess_char = get_input_char();
        if let Ok(guess_char) = guess_char {
            guess_word_chars.push(guess_char);

            if secret_word_chars.contains(&guess_char) {
                for (index, c) in secret_word_chars.iter().enumerate() {
                    if *c == guess_char {
                        guess_word[index] = guess_char;
                    }
                }
            } else {
                println!("Sorry, that letter is not in the word");
                guess_count -= 1;
            }
            println!();

            if secret_word_chars == guess_word {
                println!(
                    "Congratulations you guessed the secret word: {}!",
                    secret_word
                );
                break;
            }

            if guess_count == 0 {
                println!("Sorry, you ran out of guesses!");
                break;
            }
        }
    }
}

fn print_guess_word(guess_word: &Vec<char>) {
    print!("The word so far is ");
    for c in guess_word.iter() {
        print!("{}", c);
    }
    println!();
}

fn print_guess_chars(guess_word_chars: &str) {
    println!(
        "You have guessed the following letters: {}",
        guess_word_chars
    );
}

fn get_input_char() -> Result<char, String> {
    print!("Please guess a letter: ");
    io::stdout().flush().expect("Error flushing stdout.");
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            if guess.len() != 2 {
                get_input_char();
            }

            let guess_char = guess.chars().next().unwrap();
            Ok(guess_char)
        }
        Err(error) => Err("Error reading line.".to_string()),
    }
}
