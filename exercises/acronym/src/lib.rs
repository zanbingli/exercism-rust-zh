
extern crate inflector;
use inflector::Inflector;

pub fn abbreviate(phrase: &str) -> String {
    phrase.to_title_case().chars().filter(|x|x.is_ascii_uppercase()).collect()
}
