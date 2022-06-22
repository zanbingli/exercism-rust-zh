pub struct Luhn {
    data: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(self.data.as_str())
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
    where T: ToString {
    fn from(input: T) -> Self {
        Luhn {
            data: input.to_string(),
        }
    }
}

fn is_valid(code: &str) -> bool {
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
