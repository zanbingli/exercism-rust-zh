pub fn collatz(n: u64) -> Option<u64> {
    if n== 0 {
        return None;
    }
    let mut bz = 0;
    let mut va = n;
    loop {
        if  va == 1{
            return Some(bz);
        }
        if va %2 == 0 {
            va /= 2;
        }else {
            va = va * 3 + 1;
        }
        bz += 1;
    }
}
