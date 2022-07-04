extern crate itertools;

use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() { return 0; }
    if lines[0].is_empty() { return 0; }
    let row = lines.len();
    let col = lines[0].len();

    let mut vec = vec![];
    for i in 0..row {
        for j in 0..col {
            if let Some('+') = lines[i].chars().nth(j) {
                vec.push((i, j));
            }
        }
    }
    vec.into_iter()
        .combinations(4)
        .filter(|x| is_valid_rect(x, lines))
        .count() as u32
}

fn is_valid_rect(vec: &Vec<(usize, usize)>, lines: &[&str]) -> bool {
    if vec.len() != 4 { return false; }
    let p1: &(usize, usize) = &vec[0];
    let p2: &(usize, usize) = &vec[1];
    let p3: &(usize, usize) = &vec[2];
    let p4: &(usize, usize) = &vec[3];

    if p1.0 == p2.0 && p1.1 == p3.1 && p2.1 == p4.1 && p3.0 == p4.0 {
        for i in p1.0 + 1..p3.0 {
            match lines[i].chars().nth(p1.1) {
                Some('-')=>return false,
                Some(' ')=>return false,
                _=>(),
            }
            match lines[i].chars().nth(p2.1) {
                Some('-')=>return false,
                Some(' ')=>return false,
                _=>(),
            }
        }
        return true;
    }
    false
}
