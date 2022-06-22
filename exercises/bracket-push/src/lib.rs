const BRACKET_STR: &str = "[]{}()";

pub fn brackets_are_balanced(string: &str) -> bool {
    string.chars().filter(|&x| BRACKET_STR.contains(x))
        .fold(String::new(), |mut acc, c| {
            match c {
                ')' => {
                    if acc.ends_with('(') {
                        acc.pop();
                    }else {
                        acc.push(c);
                    }
                },
                ']' => {
                    if acc.ends_with('[') {
                        acc.pop();
                    }else {
                        acc.push(c);
                    }
                },
                '}' => {
                    if acc.ends_with('{') {
                        acc.pop();
                    }else {
                        acc.push(c);
                    }
                },
                _ => acc.push(c),
            }
            acc
        }).is_empty()
}
