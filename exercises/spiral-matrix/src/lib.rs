pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut rt = vec![vec![0; size as usize]; size as usize];
    if size == 0 {
        return rt;
    }
    let mut pos = (0, 0, Dir::East);
    let mut num = 2;
    rt[0][0] = 1;
    loop {
        if let Some(v) = get_new_pos(&pos, &rt) {
            rt[v.0 as usize][v.1 as usize] = num;
            pos = v.clone();
            num += 1;
        } else {
            break;
        }
    }
    rt
}

#[derive(Clone)]
pub enum Dir {
    East,
    South,
    West,
    North,
}

fn get_new_pos(pos: &(i32, i32, Dir), matrix: &Vec<Vec<u32>>) -> Option<(i32, i32,
                                                                         Dir)> {
    let len = matrix.len() as i32;
    match pos.2 {
        Dir::East => {
            if is_valid_pos(&(pos.0, pos.1 + 1), len, matrix) {
                return Some((pos.0, pos.1 + 1, Dir::East));
            }
            if is_valid_pos(&(pos.0 + 1, pos.1), len, matrix) {
                return Some((pos.0 + 1, pos.1, Dir::South));
            }
            if is_valid_pos(&(pos.0, pos.1 - 1), len, matrix) {
                return Some((pos.0, pos.1 - 1, Dir::West));
            }
            if is_valid_pos(&(pos.0 - 1, pos.1), len, matrix) {
                return Some((pos.0 - 1, pos.1, Dir::North));
            }
        }
        Dir::South => {
            if is_valid_pos(&(pos.0 + 1, pos.1), len, matrix) {
                return Some((pos.0 + 1, pos.1, Dir::South));
            }
            if is_valid_pos(&(pos.0, pos.1 - 1), len, matrix) {
                return Some((pos.0, pos.1 - 1, Dir::West));
            }
            if is_valid_pos(&(pos.0 - 1, pos.1), len, matrix) {
                return Some((pos.0 - 1, pos.1, Dir::North));
            }
            if is_valid_pos(&(pos.0, pos.1 + 1), len, matrix) {
                return Some((pos.0, pos.1 + 1, Dir::East));
            }
        }
        Dir::West => {
            if is_valid_pos(&(pos.0, pos.1 - 1), len, matrix) {
                return Some((pos.0, pos.1 - 1, Dir::West));
            }
            if is_valid_pos(&(pos.0 - 1, pos.1), len, matrix) {
                return Some((pos.0 - 1, pos.1, Dir::North));
            }
            if is_valid_pos(&(pos.0, pos.1 + 1), len, matrix) {
                return Some((pos.0, pos.1 + 1, Dir::East));
            }
            if is_valid_pos(&(pos.0 + 1, pos.1), len, matrix) {
                return Some((pos.0 + 1, pos.1, Dir::South));
            }
        }
        Dir::North => {
            if is_valid_pos(&(pos.0 - 1, pos.1), len, matrix) {
                return Some((pos.0 - 1, pos.1, Dir::North));
            }
            if is_valid_pos(&(pos.0, pos.1 + 1), len, matrix) {
                return Some((pos.0, pos.1 + 1, Dir::East));
            }
            if is_valid_pos(&(pos.0 + 1, pos.1), len, matrix) {
                return Some((pos.0 + 1, pos.1, Dir::South));
            }
            if is_valid_pos(&(pos.0, pos.1 - 1), len, matrix) {
                return Some((pos.0, pos.1 - 1, Dir::West));
            }
        }
    }
    None
}

fn is_valid_pos(pos: &(i32, i32), len: i32, matrix: &Vec<Vec<u32>>) -> bool {
    pos.0 >= 0 && pos.0 < len && pos.1 >= 0 && pos.1 < len && matrix[pos.0 as usize][pos.1 as usize]
        == 0
}
