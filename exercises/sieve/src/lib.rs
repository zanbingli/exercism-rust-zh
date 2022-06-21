pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound).filter(|&x|is_prime(x)).collect()
}

fn is_prime(num:u64)->bool {
    if num < 2 {return false; }
    if num == 2 || num == 3 {return true; }
    if num % 2 == 0 {return false; }
    let mut a = 3;
    while a* a <= num {
        if num % a == 0 {return false; }
        a+= 2;
    }
    true
}