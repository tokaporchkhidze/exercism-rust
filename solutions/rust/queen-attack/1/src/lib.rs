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
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.file == other.position.file || self.position.rank == other.position.rank {
            return true;
        }
        let mut rank = self.position.rank;
        let mut file = self.position.file;
        // check on left side of the queen
        if other.position.rank < self.position.rank {
            // check on top left of the queen
            if other.position.file < self.position.file {
                while rank > 0 && file > 0 {
                    rank -= 1;
                    file -= 1;
                    if rank == other.position.rank && file == other.position.file {
                        return true;
                    }
                }
            } else {
                while rank > 0 && file < 7 {
                    rank -= 1;
                    file += 1;
                    if rank == other.position.rank && file == other.position.file {
                        return true;
                    }
                }
            }
        } else {
            if other.position.rank > self.position.rank {
                if other.position.file < self.position.file {
                    while rank < 7 && file > 0 {
                        rank += 1;
                        file -= 1;
                        if rank == other.position.rank && file == other.position.file {
                            return true;
                        }
                    }
                } else {
                    while rank < 7 && file < 7 {
                        rank += 1;
                        file += 1;
                        if rank == other.position.rank && file == other.position.file {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
