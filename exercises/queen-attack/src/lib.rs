#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }
        Some(ChessPosition {
            rank,
            file,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ps1 = &self.position;
        let ps2 = &other.position;
        if ps1.file == ps2.file || ps1.rank == ps2.rank {
            return true;
        }
        (ps1.file-ps2.file).abs() == (ps1.rank-ps2.rank).abs()
    }
}

