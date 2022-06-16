pub fn is_leap_year(year: i32) -> bool {
    if year%4 == 0 {
        if year %100 == 0 && year % 400 !=0 {
            return false;
        }
        return true;
    }
    false
}
