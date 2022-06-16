pub fn factors(n: u64) -> Vec<u64> {
    let mut rt = vec![];
    let mut num = n;
    let mut i = 2;
    while num > 1 {
        while num %i == 0 {
            rt.push(i);
            num /= i;
        }
        i += 1;
    }
    rt
}
