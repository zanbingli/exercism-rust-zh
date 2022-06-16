pub fn nth(n: u32) -> u32 {
    let mut index = 0;
    let mut num = 2;
    loop {
        if is_prime(num) {
            if index == n {
                return num;
            }
            index += 1;
        }
        num += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3{
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut a = 3;
    while a * a <= n {
        if n % a == 0 {
            return false;
        }
        a += 2;
    }
    return true;
}
