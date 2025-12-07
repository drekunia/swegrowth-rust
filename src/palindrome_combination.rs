pub fn count_longest_palindrome_combination(input: &str) -> usize {
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

    if length < input.len() as usize {
        length += 1;
    }

    length
}
