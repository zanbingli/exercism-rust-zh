pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None; }
    b_find(0, array.len() - 1, array, key)
}

fn b_find(st: usize, ed: usize, array: &[i32], key: i32) -> Option<usize> {
    if st > ed { return None; }
    let mi = (st + ed) / 2;

    if array[mi] == key {
        return Some(mi);
    }

    if mi > st {
        if let Some(v) = b_find(st, mi - 1, array, key) {
            return Some(v);
        }
    }
    if mi < ed {
        if let Some(v) = b_find(mi + 1, ed, array, key) {
            return Some(v);
        }
    }
    None
}