#[derive(Debug)]
pub struct Word {
    pub letters: Vec<char>,
    pub count: usize,
}

pub fn to_word_struct(word: &str) -> Word {
    let letters: Vec<char> = word.chars().collect();

    Word {
        letters: letters.clone(),
        count: letters.len(),
    }
}
