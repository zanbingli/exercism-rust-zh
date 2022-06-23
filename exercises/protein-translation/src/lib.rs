use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).map(|&x|x)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let slt:Vec<String> = rna.chars().collect::<Vec<char>>().chunks(3)
            .map(|x| x.iter().cloned().collect::<String>()).collect();
        if slt.is_empty() || slt.iter().any(|x|!self.pairs.contains_key(x.as_str())) {
            return None;
        }
        let mut vec = vec![];
        for v in slt.iter() {
            if let Some(&"stop codon") = self.pairs.get(v.as_str()) {
                break;
            }else {
                vec.push(*self.pairs.get(v.as_str()).unwrap());
            }
        }
        Some(vec)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs:pairs.into_iter().collect(),
    }
}

