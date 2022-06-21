pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|x|get_code(x,key)).collect()
}
fn get_code(c:char,key:i8)->char{
    if !c.is_alphabetic() {return c; }
    if c.is_lowercase(){
        return if c as u8 + key as u8 - 'a' as u8 >= 26 {
            (c as u8 + key as u8 - 26) as char
        } else {
            (c as u8 + key as u8) as char
        }
    }
    return if c as u8 + key as u8 - 'A' as u8 >= 26 {
        (c as u8 + key as u8 - 26) as char
    } else {
        (c as u8 + key as u8) as char
    }
}
