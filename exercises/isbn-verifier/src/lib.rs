/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let t_str: Vec<char> =
        isbn.chars().filter(|&x| x.is_digit(10) || x == 'X' || x == 'x').collect();
    if t_str.len() != 10 { return false; }
    if t_str[0..9].iter().any(|x| !x.is_digit(10)) { return false; }

    t_str.iter().enumerate()
        .map(|(i, v)| (10 - i, v))
        .fold(0, |mut gcc, (dex, &va)| {
            if va.is_digit(10) {
                gcc += va.to_digit(10).unwrap()* dex as u32;
            }else if va == 'x' || va == 'X' {
                gcc += 10*dex as u32;
            }
            gcc
        }) % 11 == 0
}
