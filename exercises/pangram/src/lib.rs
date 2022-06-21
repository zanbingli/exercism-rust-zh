use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
static STR: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(sentence: &str) -> bool {
    let mp:HashMap<char,usize> = sentence.to_lowercase().chars()
        .filter(|&x| STR.contains(x))
        .enumerate().map(|(a,b)|(b,a))
        .collect();
    mp.len() == 26
}
