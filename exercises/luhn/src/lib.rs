/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|x| !x.is_digit(10) && x != ' ') {
        return false;
    }
    if code.chars().filter(|x| x.is_digit(10)).count() < 2 {
        return false;
    }
    code.chars().filter(|x| x.is_digit(10)).rev().enumerate()
        .map(|(i, v)| {
            let (ii, mut vv) = (i, v.to_digit(10).unwrap());
            if i % 2 == 1 {
                vv = vv * 2;
                if vv > 9 {
                    vv -= 9;
                }
            }
            (ii, vv)
        })
        .fold(0, |acc, (_i, v)| {
            acc + v
        }) % 10 == 0
}
