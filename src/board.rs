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
        1u64 << self.0 & 0b00111111
    }
}

pub struct Board {
    black: Player,
    white: Player,
    en_passant_target: EnPassantTarget,
    half_counter: u8,
    full_counter: u32 //pretty sure it's impossible to have a chess game go longer than 4 billion moves, but we'll see
}