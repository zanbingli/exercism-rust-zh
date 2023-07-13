pub fn lowest_price(books: &[u32]) -> u32 {
    let mut rt:Vec<Vec<u32>> = vec![];

    for n in books {
        add_to_vec(*n, &mut rt);
    }
    caculate(&rt)
}

fn add_to_vec(num:u32,ans:&mut Vec<Vec<u32>>) {
    if ans.is_empty() || ans.iter().all(|v|v.contains(&num)) {
        ans.push(vec![num]);
    }else {
        let mut vec_ways:Vec<Vec<Vec<u32>>> = vec![];

        for i in 0..ans.len() {
            if !ans.get(i).unwrap().contains(&num) {
                ans.get_mut(i).unwrap().push(num);
                vec_ways.push(ans.clone());
                ans.get_mut(i).unwrap().pop();
            }
        }

        if let Some(s) = vec_ways.iter().min_by(|x,y|{
            caculate(x).cmp(&caculate(y))
        }) {
            *ans = s.clone();
        }
    }
}

fn caculate(solution:&Vec<Vec<u32>>) -> u32 {
    solution.iter()
    .map(|x|get_money(x.len())).sum()
}

fn get_money(num:usize)->u32 {
    match num {
        0=>0,
        1=>800,
        2=>1520,
        3=>2160,
        4=>2560,
        5=>3000,
        _=>3000+get_money(num-5),
    }
}
