#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sm = (1..num).filter(|x| num % x == 0).sum::<u64>();
    return if sm == num {
        Some(Classification::Perfect)
    } else if sm > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    };
}
