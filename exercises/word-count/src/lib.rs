use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut mp = HashMap::new();
    for s in words.to_lowercase().split(|x:char| !x.is_alphanumeric()) {
        if !s.is_empty() {
            *mp.entry(s.to_string()).or_insert(0) += 1;
        }
    }
    mp
}
