// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let nums: Vec<&str> = input.split("\n").collect();
    if nums.len() % 4 != 0 { return Err(Error::InvalidRowCount(nums.len())); }
    let mut cols = 0;
    for it in nums.iter() {
        if cols == 0 {
            cols = it.len();
        }
        if it.len() % 3 != 0 || it.len() != cols {
            return Err(Error::InvalidColumnCount(it.len()));
        }
    }
    let mut rt: Vec<String> = vec![];
    for it in nums.chunks(4) {
        if it.len() == 4 {
            if let Some(v) = it.first() {
                let mut index = 0;
                let mut ts = String::new();
                while index < v.len() {
                    ts.push(
                        convert_character(&it[0][index..index + 3].chars()
                            .chain(it[1][index..index + 3].chars())
                            .chain(it[2][index..index + 3].chars())
                            .chain(it[3][index..index + 3].chars()).collect()));
                    index += 3;
                }
                rt.push(ts);
            }
        }
    }
    Ok(rt.join(","))
}

fn convert_character(input: &Vec<char>) -> char {
    if &input[..] == [' ', '_', ' ', '|', ' ', '|', '|', '_', '|', ' ', ' ', ' '] {
        '0'
    } else if &input[..] == [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '1'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', '|', '_', ' ', ' ', ' ', ' '] {
        '2'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '3'
    } else if &input[..] == [' ', ' ', ' ', '|', '_', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '4'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', ' ', '_', '|', ' ', ' ', ' '] {
        '5'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', '|', '_', '|', ' ', ' ', ' '] {
        '6'
    } else if &input[..] == [' ', '_', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '7'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', '|', '_', '|', ' ', ' ', ' '] {
        '8'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '9'
    } else {
        '?'
    }
}