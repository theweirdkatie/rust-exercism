#[derive(Debug)]
pub struct ChessPosition{
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self{ rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // check row (rank)
        self.0.rank == other.0.rank || 
        // check column (file)
        self.0.file == other.0.file ||
        // check diagonal (difference between x's and y's are the same)
        (self.0.file-other.0.file).abs() == (self.0.rank-other.0.rank).abs()
    }
}
