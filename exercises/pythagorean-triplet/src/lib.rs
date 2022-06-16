pub fn find() -> Option<u32> {
    for i in 1..1000 {
        for j in i..1000 - i {
            let k = 1000 - i - j;
            if i * i + j * j == k * k {
                return Some(i * j * k);
            }
        }
    }
    None
}
