/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().map(|x|get_score(x)).sum()
}

fn get_score(c: char) -> u64 {
    return if "aeioulnrst".contains(c) {
        1
    } else if "dg".contains(c) {
        2
    } else if "bcmp".contains(c) {
        3
    } else if "fhvwy".contains(c) {
        4
    } else if "k".contains(c) {
        5
    } else if "jx".contains(c) {
        8
    } else if "qz".contains(c) {
        10
    } else {
        0
    };
}
