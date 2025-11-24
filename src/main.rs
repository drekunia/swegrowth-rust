use std::env;
use std::io;
use std::io::Write;

const DEFAULT_LETTERS: &str = "palindrome";

struct Word {
    letters: Vec<char>,
    count: usize
}

#[derive(Debug, PartialEq)]
enum CaseMode {
    CaseSensitive,
    CaseInsensitive
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = CaseMode::CaseSensitive;

    if args.len() > 1 {
        if args.len() > 2 {
            println!("Too many arguments. Only --case-insensitive or --case-insensitive allowed.");
            return;
        } else {
           mode = match args[1].as_str() {
                "--case-sensitive" => CaseMode::CaseSensitive,
                "--case-insensitive" => CaseMode::CaseInsensitive,
                
                invalid_arg => {
                    println!("Unknown argument '{}'. Valid options are --case-sensitive or --case-insensitive.", invalid_arg);
                    return;
                }
            }; 
        }
    }

    print!("Enter letters ({:?}): ", mode);
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input.");

    let mut word = match mode {
        CaseMode::CaseInsensitive => input.to_lowercase().trim().to_string(),
        CaseMode::CaseSensitive => input.trim().to_string(),
    };
    
    if word.is_empty() {
        println!("No letters found. Using the word \"{DEFAULT_LETTERS}\" instead.");
        word = DEFAULT_LETTERS.to_string();
    } else if !word.chars().all(|c| c.is_alphabetic()) {
        let word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        println!("Input contains non-alphabetic characters. Filtered input, using \"{word}\" instead.");
    }

    let longest_palindrome_segment = get_longest_palindrome_segment(&word);
    let longest_palindrome_combination_count = count_longest_palindrome_combination(&word);

    println!("Longest palindrome segment in the given phrase/word is {} letters long (i.e. \"{}\").",
        longest_palindrome_segment.len(), longest_palindrome_segment);

    println!("Longest palindrome that can be made from the combination of given letters is {} letters long.",
        longest_palindrome_combination_count);
}

fn count_longest_palindrome_combination(input: &str) -> i32 {
    let mut length = 0;
    let mut seen = [false; 128];

    for b in input.bytes() {
        let ascii = b as usize;
        
        if seen[ascii] {
            length += 2;
            seen[ascii] = false;
        } else {
            seen[ascii] = true;
        }
    }

    if length < input.len() as i32 {
        length += 1;
    }

    length
}

fn get_longest_palindrome_segment(input: &str) -> String {
    let word_struct = to_word_struct(&input);
    let mut longest_palindrome = String::new();

    for i in 0..word_struct.count {
        for j in i + 1..=word_struct.count {
            let word: String = word_struct.letters[i..j].iter().collect();

            if (word.len() > longest_palindrome.len()) && is_palindrome(&word) {
    longest_palindrome = word;
            }
        }
    }

    longest_palindrome
}

fn is_palindrome(word: &str) -> bool {
    let word_struct = to_word_struct(&word);

    for i in 0..word_struct.count / 2 {
        if word_struct.letters[i] != word_struct.letters[word_struct.count - 1 - i] {
            return false;
        }
    }

    true
}

fn to_word_struct(word: &str) -> Word {
    let letters: Vec<char> = word.chars().collect();

    Word {
        letters: letters.clone(),
        count: letters.len(),
    }
}

