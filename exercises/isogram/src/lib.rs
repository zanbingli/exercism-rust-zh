use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    candidate.to_lowercase().chars().filter(|x|x.is_alphabetic()).collect::<Vec<_>>().len()
    == candidate.to_lowercase().chars().filter(|x|x.is_alphabetic()).collect::<HashSet<_>>().len()
}
