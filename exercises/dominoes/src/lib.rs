extern crate itertools;

use itertools::Itertools;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let vec: Vec<Vec<(u8, u8)>> = input.iter().clone().permutations(input.len())
        .map(|x| x.iter().map(|&v| v.clone()).collect()).collect();

    for v in vec {
        if let Some(vv) = valid_sq(&v) {
            return Some(vv);
        }
    }
    None
}

fn valid_sq(vec: &Vec<(u8, u8)>) -> Option<Vec<(u8, u8)>> {
    if !vec.is_empty() {
        let (rt, ve, rt2, ve2) = vec.iter()
            .fold((vec![], vec![], vec![], vec![]), |(mut rt, mut acc, mut rt2, mut acc2), (i, j)| {
                //first element
                if acc.is_empty() && acc2.is_empty(){
                    acc.push(*i);
                    acc.push(*j);
                    rt.push((*i,*j));

                    acc2.push(*j);
                    acc2.push(*i);
                    rt2.push((*j,*i));

                } else {
                    if acc.last().unwrap() == i {
                        acc.pop();
                        acc.push(*j);
                        rt.push((*i,*j));
                    } else if acc.last().unwrap() == j {
                        acc.pop();
                        acc.push(*i);
                        rt.push((*j,*i));
                    }
                    if acc2.last().unwrap() == i {
                        acc2.pop();
                        acc2.push(*j);
                        rt2.push((*i,*j));
                    } else if acc2.last().unwrap() == j {
                        acc2.pop();
                        acc2.push(*i);
                        rt2.push((*j,*i));
                    }
                }
                (rt, acc, rt2, acc2)
            });
        if ve.len() == 2 && ve.first().unwrap() == ve.last().unwrap() && rt.len() == vec.len(){
            return Some(rt);
        } else if ve2.len() == 2 && ve2.first().unwrap() == ve2.last().unwrap() && rt.len() == vec.len(){
            return Some(rt2);
        }
    }
    None
}



