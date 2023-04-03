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
use std::collections::HashMap;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

// fn output() -> {
//     println!("You have guessed the following letters: ")
//     println!("You have guesses left")
//     println!("Please guess a letter: ")
//     if {

//     } else {

//     }
// }

fn read_a_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c : char = match input.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid input");
            return '\0';
        }
    };
    return c;
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    let mut cnt = 5;
    let mut c : char;
    let mut input_chars: Vec<char> = Vec::new();
    let mut matched_str : Vec<u8> = Vec::new();
    let mut success :bool = false;
    for i in secret_word_chars.iter() {
        matched_str.push(0);
    }
    println!("Welcome to CS110L Hangman!");
    while cnt > 0 {
        let mut str1 = String::new();
        for (i, b) in matched_str.iter().enumerate() {
            if *b == 1 {
                str1.push(secret_word_chars[i]);
            } else {
                str1.push('-');
            }
        }
        println!("The word so far is {}", str1);
        let str2: String = input_chars.iter().collect();
        println!("You have guessed the following letters: {}", str2);
        println!("You have {} guesses left", cnt);
        println!("Please guess a letter: ");
        c = read_a_char();
        let mut flag : bool = false;
        for i in secret_word_chars.iter() {
            if c == *i {
                flag = true;
                break;
            }
        }
        if flag == false {
            println!("Sorry, that letter is not in the word");
            cnt = cnt - 1;
        }
        input_chars.push(c);
        success = true;
        for c in input_chars.iter() {
            for (i, b) in secret_word_chars.iter().enumerate() {
                if *b == *c && matched_str[i] == 0 {
                    matched_str[i] = 1;
                    break;
                }
            }
        }
        for i in matched_str.iter() {
            if *i == 0 {
                success = false;
                break;
            }
        }
        if cnt == 0 && !success {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        if success {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
    }
}
