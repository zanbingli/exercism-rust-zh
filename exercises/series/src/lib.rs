pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut rt = vec![];
    if digits.len()< len {
        return rt;
    }
    for i in 0..=digits.len()-len {
        rt.push(digits[i..i+len].to_owned());
    }
    rt
}
