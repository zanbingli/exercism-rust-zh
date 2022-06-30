pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    word: String,
    meaning: String,
    value: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            word: String::new(),
            meaning: String::new(),
            value: vec![],
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.value.clone()
    }

    pub fn push(&mut self, va: &str) -> ForthResult {
        if let Ok(v) = va.parse::<Value>() {
            self.value.push(v);
            return Ok(());
        }

        match va.to_lowercase().as_str() {
            "+" => {
                if self.value.len() > 1 {
                    let v = self.value.pop().unwrap();
                    if let Some(vv) = self.value.last_mut() {
                        *vv += v;
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "-" => {
                if self.value.len() > 1 {
                    let v = self.value.pop().unwrap();
                    if let Some(vv) = self.value.last_mut() {
                        *vv -= v;
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "*" => {
                if self.value.len() > 1 {
                    let v = self.value.pop().unwrap();
                    if let Some(vv) = self.value.last_mut() {
                        *vv *= v;
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "/" => {
                if self.value.len() > 1 {
                    let v = self.value.pop().unwrap();
                    if let Some(vv) = self.value.last_mut() {
                        if v == 0 {
                            return ForthResult::Err(Error::DivisionByZero);
                        }
                        *vv /= v;
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "dup" => {
                if !self.value.is_empty() {
                    let v = self.value.last().cloned();
                    if v.is_some() {
                        self.value.push(v.unwrap());
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "drop" => {
                if !self.value.is_empty() {
                    if let Some(_vv) = self.value.pop() {
                        return Ok(());
                    }
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "swap" => {
                if self.value.len() > 1 {
                    let (i, j) = (self.value.len() - 1, self.value.len() - 2);
                    self.value.swap(i, j);
                    return Ok(());
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            "over" => {
                if self.value.len() > 1 {
                    let i = self.value.len() - 2;
                    self.value.push(self.value[i].clone());
                    return Ok(());
                } else {
                    return Result::Err(Error::StackUnderflow);
                }
            }
            _ => {}
        }
        ForthResult::Err(Error::UnknownWord)
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        if input.trim().starts_with(':') {
            if !input.contains(';') { return ForthResult::Err(Error::InvalidWord); }
        } else {
            if input.contains(self.word.as_str()) {
                let vv: String =
                    input.replace(self.word.as_str(), self.meaning.as_str());
                let vec: Vec<&str> =
                    vv.as_str().split(|x: char| !(x.is_alphanumeric()
                        || "+-*/".contains(x))).filter(|x| !x.is_empty())
                        .collect();
                for it in vec {
                    if let Err(e) = self.push(it) {
                        return ForthResult::Err(e);
                    } else {
                        continue;
                    }
                }
            } else {
                let vec: Vec<&str> =
                    input.split(|x: char| !(x.is_alphanumeric()
                        || "+-*/".contains(x))).filter(|x| !x.is_empty())
                        .collect();
                for it in vec {
                    if let Err(e) = self.push(it) {
                        return ForthResult::Err(e);
                    } else {
                        continue;
                    }
                }
            }
        }

        ForthResult::Ok(())
    }
}
