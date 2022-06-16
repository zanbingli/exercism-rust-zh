
pub fn build_proverb(list: Vec<&str>) -> String {
    let mut rt = String::new();
    if list.is_empty() {
        return rt;
    }
    for i in 0..list.len()-1 {
        rt += (format!("For want of a {} the {} was lost.\n",
                        list.get(i).unwrap(),list.get(i+1).unwrap())).as_str();
    }
    if !list.is_empty() {
        rt += (format!("And all for the want of a {}.",list.first().unwrap())).as_str();
    }
    rt
}
