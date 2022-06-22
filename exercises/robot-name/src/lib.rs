
extern crate rand;
use rand::{Rng};

const NUM_STR:&[u8] = b"0123456789";
const CHAR_STR:&[u8] = b"ABCDEFGHIGKLMNOPQRSTUVWXYZ";

pub struct Robot{
    name:String,
}

impl Robot {
    pub fn new() -> Self {
        Robot{
            name:generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        loop {
            let n_name = generate_name();
            if !self.name.eq(&n_name) {
                self.name = n_name;
                break;
            }
        }
    }
}

fn generate_name()->String {
    let mut rng = rand::thread_rng();
    (0..2).map(|_v|{
        let index:usize = rng.gen_range(0..26);
        CHAR_STR[index] as char
    }).collect::<String>() + (0..3).map(|_x|{
        let index:usize = rng.gen_range(0..10);
        NUM_STR[index] as char
    }).collect::<String>().as_str()
}
