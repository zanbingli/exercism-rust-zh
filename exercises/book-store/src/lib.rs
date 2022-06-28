use std::collections::HashSet;

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut bks: Vec<u32> = books.iter().map(|x| x.clone()).collect();
    try_fetch(&mut bks)
}

fn try_fetch(bks: &mut Vec<u32>) -> u32 {
    let v = fetch(bks);
    if v.is_empty() { return 0; }
    v.iter().map(|x| get_group_cost(&x)).sum()
}

fn fetch(bks: &mut Vec<u32>) -> Vec<Vec<u32>> {
    let set: HashSet<u32> = bks.iter().cloned().collect();
    let mut vec = vec![];
    let most_ele = set.iter().map(|x| count_element(x, bks)).max().unwrap_or(0);
    for _ii in 0..most_ele {
        vec.push(can_fetch(bks));
    }
    arrange_groups(&mut vec);
    vec
}

fn arrange_groups(groups: &mut Vec<Vec<u32>>) {
    if groups.is_empty() { return; }
    loop {
        let l = groups.iter().map(|x| x.len()).max().unwrap();
        let s = groups.iter().map(|x| x.len()).min().unwrap();
        if l - s > 1 {
            //削峰填谷
            let mut g1 =
                groups.iter().find(|x| x.len() == l).unwrap().clone();
            let mut g2 = groups.iter().find(|x| x.len() == s).unwrap().clone();
            if arrange_two(&mut g1, &mut g2) {
                let g11 =
                    groups.iter_mut().find(|x| x.len() == l).unwrap();
                *g11 = g1.clone();
                let g22 = groups.iter_mut().find(|x| x.len() == s).unwrap();
                *g22 = g2.clone();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn arrange_two(group1: &mut Vec<u32>, group2: &mut Vec<u32>) -> bool {
    if group1.len() == 5 && group2.len() == 3 {
        let set: HashSet<u32> = group1.iter().cloned().collect();
        for it in set.iter() {
            if !group2.contains(it) {
                group2.push(it.clone());
                remove_one_element(it, group1);
                break;
            }
        }
        return true;
    }

    false
}

fn can_fetch(bks: &mut Vec<u32>) -> Vec<u32> {
    let set: HashSet<u32> = bks.iter().cloned().collect();
    let mut vec = vec![];
    for s in set.iter() {
        if bks.contains(s) {
            vec.push(s.clone());
            remove_one_element(s, bks);
        }
    }
    vec
}

fn count_element(bk: &u32, bks: &Vec<u32>) -> usize {
    bks.iter().filter(|&x| x == bk).count()
}

fn remove_one_element(bk: &u32, bks: &mut Vec<u32>) {
    for i in 0..bks.len() {
        if bks[i] == *bk {
            bks.remove(i);
            return;
        }
    }
}

// 每个分组的费用
fn get_group_cost(group: &Vec<u32>) -> u32 {
    return match group.iter().map(|&x| x > 0_u32).count() {
        0 => 0,
        1 => 800,
        2 => 1520,
        3 => 2160,
        4 => 2560,
        5 => 3000,
        _ => 0,
    };
}