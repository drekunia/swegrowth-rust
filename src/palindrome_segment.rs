use crate::word::to_word_struct;

pub fn get_longest_palindrome_segment(input: &str) -> String {
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

