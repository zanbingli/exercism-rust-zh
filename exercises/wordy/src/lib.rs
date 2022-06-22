

const OPERATE_STR:&[&str] = &["plus", "minus", "multiplied", "divided"];
pub struct WordProblem {
    value: i32,
    operate: Operate,
}
#[derive(PartialOrd, PartialEq)]
pub enum Operate {
    Plus,
    Minus,
    Multiply,
    Divide,
    None,
}

pub fn answer(command: &str) -> Option<i32> {
    let mut wp = WordProblem {
        value:0,
        operate:Operate::None,
    };
    let vec:Vec<&str> =
        command.split(|x|x==' ' || x == '?').filter(|&f| !f.is_empty())
            .filter(|&x| OPERATE_STR.contains(&x) || x.parse::<i32>().is_ok()).collect();
    for v in vec.iter() {
        if v.parse::<i32>().is_ok() {
            wp.push_va(v);
        }else {
            wp.push_op(v);
        }
    }
    if wp.operate != Operate::None {
        return Some(wp.value);
    }
    None
}

impl WordProblem {
    fn push_va(&mut self, vas: &str) {
        let va = vas.parse().unwrap_or(0);
        match self.operate {
            Operate::Plus => self.value += va,
            Operate::Minus => self.value -= va,
            Operate::Multiply => self.value *= va,
            Operate::Divide => self.value /= va,
            Operate::None=>self.value = va,
        }
    }
    fn push_op(&mut self, op: &str) {
        self.operate = match op {
            "plus"=>Operate::Plus,
            "minus"=>Operate::Minus,
            "multiplied"=>Operate::Multiply,
            "divided"=>Operate::Divide,
            _ => Operate::None,
        };
    }
}
