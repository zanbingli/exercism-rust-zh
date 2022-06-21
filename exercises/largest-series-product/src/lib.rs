#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(v) = string_digits.chars().find(|x| !x.is_digit(10)) {
        return Err(Error::InvalidDigit(v));
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if let Some(mx) = (0..=string_digits.len() - span).map(|x| &string_digits[x..(x + span)])
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap())
            .fold(1_u64, |acc, u| { acc * (u as u64) })
        ).max() {
        return Ok(mx);
    }
    Err(Error::SpanTooLong)
}
