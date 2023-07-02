struct Player {
    king: u64,
    queens: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    pawns: u64,
    king_castle: bool,
    queen_castle: bool
}

enum TargetedPlayer {
    Black,
    White
}

struct EnPassantTarget(u8);

impl EnPassantTarget {
    fn targeted_player(&self) -> Option<TargetedPlayer> {
        match self.0 >> 6 {
            0 => Some(TargetedPlayer::White),
            1 => Some(TargetedPlayer::Black),
            2 | 3 => None,
            _ => unreachable!()
        }
    }
    fn targeted_square(&self) -> u64 {
        1u64 << self.0 & b00111111
    }
}

struct Board {
    black: Player,
    white: Player,
    en_passant_target: EnPassantTarget
}