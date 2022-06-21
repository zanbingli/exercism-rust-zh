/// "Encipher" with the Atbash cipher.
///

const AZ_STR: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn encode(plain: &str) -> String {
    plain.to_lowercase().chars().filter(|x|AZ_STR.contains(*x)||x.is_digit(10))
        .map(|x| if AZ_STR.contains(x) {
        get_code(x)
    } else {
        x
    }).collect::<Vec<char>>().chunks(5)
        .map(|x|x.iter().collect::<String>()).collect::<Vec<String>>().join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter(|x|AZ_STR.contains(*x)||x.is_digit(10))
        .map(|x| if AZ_STR.contains(x) {
        get_code(x)
    } else {
        x
    }).collect()
}

fn get_code(c: char) -> char {
    ('z' as u8  - c as u8 + 'a' as u8) as char
}