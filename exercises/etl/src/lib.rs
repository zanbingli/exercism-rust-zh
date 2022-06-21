use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().map(|(&k,v)|
        v.iter().map(|&c|(c.to_ascii_lowercase(),k)).collect::<BTreeMap<char,i32>>()
    ).flat_map(|m|m.into_iter()).collect()
}
