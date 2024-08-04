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

fn check(secret_word_chars: &mut Vec<char>, ch: char) -> Option<usize> {
    if let Some(res) = secret_word_chars.iter().position(|&c| c == ch) {
        secret_word_chars[res] = '-';
        Some(res)
    } else {
        None
    }
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut gueessed_chars = String::new();
    let mut track_chars = vec!['-'; secret_word.len()];
    let mut turns = NUM_INCORRECT_GUESSES;
    let mut successs = 0;

    while turns > 0 && successs < secret_word.len() {
        println!(
            "The word so far is {}",
            track_chars.iter().collect::<String>()
        );
        println!(
            "You have gueessed the following letters: {}",
            gueessed_chars
        );
        println!("You have {} guesses left", turns);
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        if input.len() > 2 {
            println!("\nPlease just input a character!\n");
            continue;
        }

        gueessed_chars.push_str(&input.trim_end_matches('\n'));
        let ch = input.chars().next().unwrap();
        if let Some(idx) = check(&mut secret_word_chars, ch) {
            track_chars[idx] = ch;
            successs += 1;
        } else {
            turns -= 1;
            println!("Sorry, that letter is not in the word\n");
        }
    }

    if successs == secret_word_chars.len() {
        println!(
            "Congratulations you guessed the secret word: {}",
            secret_word
        );
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
