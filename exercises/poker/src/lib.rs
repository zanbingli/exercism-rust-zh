use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() { return None; }
    if hands.iter().any(|&x| x.split(" ").count() != 5) { return None; }
    if hands.len() == 1 { return Some(vec![hands[0]]); }
    let mut vec: Vec<Pork> =
        hands.iter().map(|&x| Pork::new(x)).collect();
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mx = vec.last().unwrap().clone();
    Some(vec.into_iter().filter(|x| x == &mx).map(|x| x.value).collect())
}

#[derive(Clone)]
pub struct Pork<'a> {
    value: &'a str,
}

#[derive(Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum PorkType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl<'a> PartialOrd for Pork<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank_a = get_rank(&get_seq(&self.value));
        let rank_b = get_rank(&get_seq(&other.value));
        //牌型相同比大小
        return if rank_a == rank_b {
            Some(cmp_num(&get_seq(&self.value), &get_seq(&other.value), &rank_a))
        } else {
            //牌型不同比牌型
            return Some(rank_b.cmp(&rank_a));
        };
    }
}

impl<'a> PartialEq<Self> for Pork<'a> {
    fn eq(&self, other: &Self) -> bool {
        get_seq(self.value).iter().map(|(a, _b)| *a).collect::<Vec<_>>()
            == get_seq(other.value).iter().map(|(a, _b)| *a).collect::<Vec<_>>()
    }
}

impl<'a> Pork<'a> {
    pub fn new(s: &'a str) -> Self {
        Pork {
            value: s,
        }
    }
}

fn get_seq(s: &str) -> Vec<(usize, usize)> {
    s.split(" ").filter(|&x| !x.is_empty())
        .map(|x| get_num(x)).collect()
}

fn get_num(s: &str) -> (usize, usize) {
    if let Some(v) = s.chars().nth(0) {
        let a = match v {
            'A' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '1' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            _ => 0,
        };
        if s.ends_with('S') {
            return (a, 1);
        } else if s.ends_with('H') {
            return (a, 2);
        } else if s.ends_with('D') {
            return (a, 3);
        } else if s.ends_with('C') {
            return (a, 4);
        }
    }
    (0, 0)
}

fn get_rank(vec: &Vec<(usize, usize)>) -> PorkType {
    return if is_straight_flush(vec) {
        PorkType::StraightFlush
    } else if is_four_of_a_kind(vec) {
        PorkType::FourOfAKind
    } else if is_full_house(vec) {
        PorkType::FullHouse
    } else if is_flush(vec) {
        PorkType::Flush
    } else if is_straight(vec) {
        PorkType::Straight
    } else if is_three_of_a_kind(vec) {
        PorkType::ThreeOfAKind
    } else if is_tow_pair(vec) {
        PorkType::TwoPair
    } else if is_one_pair(vec) {
        PorkType::OnePair
    } else {
        PorkType::HighCard
    };
}

fn cmp_num(vec: &Vec<(usize, usize)>, other: &Vec<(usize, usize)>, rank: &PorkType) -> Ordering {
    let mut v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    let mut v2: Vec<usize> = other.iter().map(|(x, _y)| *x).collect();
    v1.sort();
    v2.sort();
    match rank {
        PorkType::StraightFlush | PorkType::Straight | PorkType::HighCard | PorkType::Flush => {
            if v1.iter().zip(v2.iter())
                .any(|(x, y)| x > y) {
                return Ordering::Greater;
            } else if v1.iter().zip(v2.iter())
                .any(|(x, y)| x < y) {
                return Ordering::Less;
            }
        }
        PorkType::FourOfAKind | PorkType::FullHouse | PorkType::ThreeOfAKind |
        PorkType::TwoPair | PorkType::OnePair => {
            let mut vv1 = vec![];
            let mut vv2 = vec![];
            for i in 1..5 {
                append_pair(&v1, i, &mut vv1);
                append_pair(&v2, i, &mut vv2);
            }
            let (s1, s2) = (get_type_score(&vv1), get_type_score(&vv2));
            if s1 > s2 {
                return Ordering::Greater;
            } else if s1 < s2 {
                return Ordering::Less;
            }
        }
    }

    Ordering::Equal
}

fn append_pair(v1: &Vec<usize>, i: usize, tar: &mut Vec<usize>) {
    for it in v1.iter() {
        if v1.iter().filter(|&x| x == it).count() == i {
            tar.insert(0, *it);
        }
    }
}

fn get_type_score(vec: &Vec<usize>) -> usize {
    vec.iter().fold(0, |acc, v| if v == &1_usize { acc * 10 + 13 } else { acc * 10 + v })
}

fn is_straight_flush(vec: &Vec<(usize, usize)>) -> bool {
    return is_flush(vec) && is_straight(vec);
}

fn is_four_of_a_kind(vec: &Vec<(usize, usize)>) -> bool {
    let v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    for i in v1.iter() {
        if v1.iter().filter(|&x| x == i).count() == 4 {
            return true;
        }
    }
    false
}

fn is_full_house(vec: &Vec<(usize, usize)>) -> bool {
    let v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    let mut rt: (bool, bool) = (false, false);
    for i in v1.iter() {
        if v1.iter().filter(|&x| x == i).count() == 3 {
            rt.0 = true;
        }
        if v1.iter().filter(|&x| x == i).count() == 2 {
            rt.1 = true;
        }
    }
    rt.0 && rt.1
}

fn is_flush(vec: &Vec<(usize, usize)>) -> bool {
    if let Some((_a, b)) = vec.first() {
        return !vec.iter().any(|(_x, y)| y != b);
    }
    false
}

fn is_straight(vec: &Vec<(usize, usize)>) -> bool {
    let mut v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    v1.sort();
    for it in v1.iter() {
        if v1.iter().filter(|&x| x == it).count() > 1 {
            return false;
        }
    }
    if v1.last().unwrap() - v1.first().unwrap() == 4
        || v1 == vec![1, 10, 11, 12, 13] {
        return true;
    }
    false
}

fn is_three_of_a_kind(vec: &Vec<(usize, usize)>) -> bool {
    let v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    for i in v1.iter() {
        if v1.iter().filter(|&x| x == i).count() == 3 {
            return true;
        }
    }
    false
}

fn is_tow_pair(vec: &Vec<(usize, usize)>) -> bool {
    let v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    let mut rt = vec![];
    for i in v1.iter() {
        if v1.iter().filter(|&x| x == i).count() == 2 {
            rt.push(*i);
        }
    }
    rt.len() == 4
}

fn is_one_pair(vec: &Vec<(usize, usize)>) -> bool {
    let v1: Vec<usize> = vec.iter().map(|(x, _y)| *x).collect();
    for i in v1.iter() {
        if v1.iter().filter(|&x| x == i).count() == 2 {
            return true;
        }
    }
    false
}




