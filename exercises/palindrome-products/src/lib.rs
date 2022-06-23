

pub type Palindrome = u64;

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    (min..=max).map(|x|{
        (min..=max).map(|x2|(x,x2)).collect::<Vec<(u64,u64)>>()
    }).flatten()
        .map(|v| v.0 * v.1)
        .filter(|x| is_palindromes(x))
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().map(|x|x.clone())
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().map(|x|x.clone())
}

fn is_palindromes(n: &Palindrome) -> bool {
    n.to_string().eq(&n.to_string().chars().rev().collect::<String>())
}
