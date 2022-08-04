// extern crate rayon;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

// use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut rt = HashMap::new();
    let p_size =
        if input.len() % worker_count == 0 { input.len() / worker_count } else {
            input.len() / worker_count + 1
        };
    // let mut threads = vec![];
    // //map
    // for sli in input.chunks(p_size) {
    //     let s = Arc::new(sli);
    //     threads.push(thread::spawn(move || -> HashMap<char, usize>{
    //         count_char(s.clone())
    //     }));
    // }
    //
    // //reduce
    // for tr in threads {
    //     let mp = tr.join().unwrap();
    //     append(&mut rt, &mp);
    // }

    //rayon solution
    // let mut raws: Vec<(&[&str], HashMap<char, usize>)> = vec![];
    //
    // for v in input.chunks(p_size) {
    //     let mmp = HashMap::new();
    //     raws.push((v, mmp));
    // }
    //
    // let raw_ref: Vec<(&[&str], &mut HashMap<char, usize>)> =
    //     raws.iter_mut().map(|(k, v)| (k.clone(), v)).collect();
    //
    // raw_ref.into_par_iter()
    //     .for_each(|(va, mut mp)| {
    //         append(&mut mp, &count_char(va));
    //     });
    //
    // //reduce
    // for (_, mp) in raws {
    //     append(&mut rt, &mp);
    // }

    // slot solution
    let mut threads = vec![];
    let (sender, receiver) = channel();
    for v in input.chunks(p_size) {
        let sd = sender.clone();
        let s:String = v.iter().flat_map(|x|x.chars()).collect();
        threads.push(
            thread::spawn(move || {
            sd.send(count_string(&s))
        }));
    }

    for _ in 0..input.chunks(p_size).len()  {
        let mp = receiver.recv().unwrap();
        append(&mut rt, &mp);
    }


    // //reduce
    // for tr in threads {
    //     tr.join().unwrap();
    // }
    rt
}

// fn count_char(sli: &[&str]) -> HashMap<char, usize> {
//     let mut rt = HashMap::new();
//     for c in sli.iter()
//         .flat_map(|x| x.chars()).collect::<String>().to_lowercase().chars()
//         .filter(|x| x.is_alphabetic()) {
//         *rt.entry(c).or_insert(0) += 1;
//     }
//     rt
// }

fn count_string(sli: &String) -> HashMap<char, usize> {
    let mut rt = HashMap::new();
    for c in sli.to_lowercase().chars()
        .filter(|x| x.is_alphabetic()) {
        *rt.entry(c).or_insert(0) += 1;
    }
    rt
}

fn append(tar: &mut HashMap<char, usize>, ap: &HashMap<char, usize>) {
    for (k, v) in ap {
        *tar.entry(*k).or_insert(0) += v;
    }
}