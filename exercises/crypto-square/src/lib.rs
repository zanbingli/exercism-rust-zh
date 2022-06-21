pub fn encrypt(input: &str) -> String {
    let v: Vec<char> = input.to_lowercase().chars().filter(|x| x.is_alphanumeric())
        .collect();
    let r = (v.len()as f32).sqrt() as usize;
    let c = if r * r == v.len() {
        r
    } else {
        r + 1
    };
    let cks:Vec<String> = v.chunks(c).map(|x|x.iter().collect()).collect();
    let mut vt:Vec<String> = vec![];
    for i in 0..c {
        let mut rt = String::new();
        for s in cks.iter(){
            if s.chars().nth(i).is_some() {
                rt.push(s.chars().nth(i).unwrap());
            }else {
                rt.push(' ');
            }
        }
        vt.push(rt);
    }
    vt.join(" ")
}
