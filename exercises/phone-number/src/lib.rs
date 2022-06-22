pub fn number(user_number: &str) -> Option<String> {
    let s:String = user_number.chars().filter(|x|x.is_digit(10)).collect();
    if s.len() == 10  {
        if s.chars().enumerate().any(|(i,v)|{
            (i == 0 || i == 3) && "01".contains(v)
        }) {
            return None;
        }
        return Some(s);
    }else if s.len() == 11 {
        if !s.starts_with('1') {return None; }
        if s.chars().enumerate().any(|(i,v)|{
            (i == 1 || i == 4) && "01".contains(v)
        }) {
            return None;
        }
        return Some(s[1..].to_string());
    }
    None
}
