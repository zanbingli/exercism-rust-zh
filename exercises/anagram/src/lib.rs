use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter(|&x| is_angrams(word, x))
        .map(|&x|x).collect()
}

fn is_angrams(s1: &str, s2: &str) -> bool {
    if s1.to_lowercase().eq(&s2.to_lowercase()) {return false; }
    s1.to_lowercase().chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    }) == s2.to_lowercase().chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}
