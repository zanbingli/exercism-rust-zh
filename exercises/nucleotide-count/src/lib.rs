use std::collections::HashMap;

const DNA_STR:&str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if  !DNA_STR.contains(nucleotide){
        return Err(nucleotide);
    }
    if let Some(x) = dna.chars().find(|v| !DNA_STR.contains(*v)) {
        return Err(x);
    }
    Ok(dna.chars().fold(0, |mut x, v| {
        if nucleotide == v {
            x += 1;
        }
        x
    }))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(x) = dna.chars().find(|v| !DNA_STR.contains(*v)) {
        return Err(x);
    }
    let mut imp: HashMap<char, usize> = HashMap::new();
    imp.insert('A',0);
    imp.insert('C',0);
    imp.insert('G',0);
    imp.insert('T',0);
    let mp: HashMap<char, usize> = dna.chars()
        .fold(imp, |mut mp, x| {
            *mp.entry(x).or_insert(0) += 1;
            mp
        });
    Ok(mp)
}
