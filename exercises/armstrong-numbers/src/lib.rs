pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    s.chars().map(|x| x.to_digit(10).unwrap().pow(s.len() as u32)).sum::<u32>() == num
}
