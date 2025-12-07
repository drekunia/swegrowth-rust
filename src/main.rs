use std::env;

mod cli;
mod palindrome_combination;
mod palindrome_segment;
mod word;

use crate::cli::*;
use crate::palindrome_combination::*;
use crate::palindrome_segment::*;

const DEFAULT_LETTERS: &str = "mississippi";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = match parse_args(&args) {
        Ok(m) => m,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let input = match prompt_input(&mode) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read input: {}", e);
            return;
        }
    };

    let mut word = match mode {
        CaseMode::CaseInsensitive => input.to_lowercase().trim().to_string(),
        CaseMode::CaseSensitive => input.trim().to_string(),
    };

    if word.is_empty() {
        word = DEFAULT_LETTERS.to_string();
        println!("No letters found. Using the word \"{word}\" instead.");
    } else if !word.chars().all(|c| c.is_alphabetic()) {
        word = word.chars().filter(|c| c.is_alphabetic()).collect();
        println!(
            "Input contains non-alphabetic characters. Filtered input, using \"{word}\" instead."
        );
    };

    let longest_palindrome_segment = get_longest_palindrome_segment(&word);
    let longest_palindrome_combination_count = count_longest_palindrome_combination(&word);

    println!(
        "Longest palindrome segment in the given phrase/word is {} letters long (i.e. \"{}\").",
        longest_palindrome_segment.len(),
        longest_palindrome_segment
    );

    println!(
        "Longest palindrome that can be made from the combination of given letters is {} letters long.",
        longest_palindrome_combination_count
    );
}
