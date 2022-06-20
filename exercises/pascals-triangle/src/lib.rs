pub struct PascalsTriangle{
    rows:u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{
            rows:row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec = vec![];
        if self.rows == 0 {return vec; }
        vec.push(vec![1]);
        for _ in 1..self.rows {
            vec.push(generate(vec.last().unwrap()));
        }
        vec
    }
}
fn generate(row:&Vec<u32>)->Vec<u32> {
    let mut v = vec![];
    v.push(1);
    for i in 0..row.len()-1 {
        v.push(row.get(i).unwrap() + row.get(i+1).unwrap())
    }
    v.push(1);
    v
}
