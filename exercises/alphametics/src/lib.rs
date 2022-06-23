extern crate itertools;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const NUMS: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let set: HashSet<char> =
        input.chars().filter(|x| x.is_alphabetic() && x.is_uppercase()).collect();

    NUMS.iter().permutations(set.len())
        .filter(|x| !x.starts_with(&[&0_u8]))
        .map(|x| {
            set.iter().map(|x| *x).zip(x.iter().map(|x| **x)).collect()
        })
        .find(|x| is_valid(x, input))
}


fn is_valid(mp: &HashMap<char, u8>, express: &str) -> bool {
    let exp: String = express.chars().map(|x| {
        if mp.contains_key(&x) {
            get_char(*mp.get(&x).unwrap())
        } else {
            x
        }
    }).collect();

    let vec_str: Vec<&str> = exp.split(|x: char| x == ' ' || x == '+' || x == '=').filter(|x| !x
        .is_empty()).collect();
    //不能0开头
    if  vec_str.iter().any(|x|x.starts_with("0")){
        return false;
    }
    let vec: Vec<u32> = vec_str.iter()
        .map(|v| v.parse().unwrap_or(0)).collect();
    if vec.last().is_some() {
       return  vec.iter().sum::<u32>() == 2*vec.last().unwrap();
    }
    false
}

fn get_char(n: u8) -> char {
    match n {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '0',
    }
}