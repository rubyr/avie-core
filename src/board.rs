#[cfg(test)]
mod test {}

struct PlayerState {
    king: u64,
    queens: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    pawns: u64,
    king_castle: bool,
    queen_castle: bool
}

impl PlayerState {
    ///function uses a quirk of binary representation to verify that there are no duplicate pieces on the board.
    /// when no bits are shared between two numbers, addition gives the same result as binary OR. by adding all 
    /// bitboards together and comparing with all bitboards OR'd together, we can ensure that a player's state is valid.
    fn is_valid(&self) -> bool {
        let mut valid = self.king.count_ones() == 1;
        let add_board: u128 = self.king as u128 + self.queens as u128 + self.bishops as u128 + self.knights as u128 + self.rooks as u128 + self.pawns as u128;
        valid &= add_board == self.all_pieces() as u128;
        valid
    }
    fn all_pieces(&self) -> u64 {
        self.king | self.queens | self.bishops | self.knights | self.rooks | self.pawns
    }
}
#[derive(PartialEq, Eq)]
enum Player {
    Black,
    White
}

///En Passant Target representation:
///  0bX0000000: active flag. if 1, there was no en passant on the previous turn, and all other bits are ignored.
///  0b0X000000: player flag. 0 for white, 1 for black.
///  0b00XXXXXX: square of valid en passant target. bitboard is obtained by shifting 1u64 by this value.
struct EnPassantTarget(u8);

impl EnPassantTarget {
    fn targeted_player(&self) -> Option<Player> {
        match self.0 >> 6 {
            0 => Some(Player::White),
            1 => Some(Player::Black),
            2 | 3 => None,
            _ => unreachable!()
        }
    }
    fn targeted_square(&self) -> u64 {
        1u64 << self.0 & 0b00111111
    }
}

pub struct BoardState {
    black: PlayerState,
    white: PlayerState,
    en_passant_target: EnPassantTarget,
    active_player: Player,
    half_counter: u8,
    full_counter: u32 //pretty sure it's impossible to have a chess game go longer than 4 billion moves, but we'll see
}

impl BoardState {

    fn all_pieces(&self) -> u64 {
        self.black.all_pieces() | self.white.all_pieces()
    }
///Pawns are able to push forward when there is no piece blocking their way
    fn pawn_single_pushes(&self) -> u64 {
        if self.active_player == Player::Black {
            let pawns = self.black.pawns;
            let other_pieces = self.all_pieces() & !pawns;
            (pawns >> 8) & (!other_pieces)
        }
        else {
            let pawns = self.white.pawns;
            let other_pieces = self.all_pieces() & !pawns;
            (pawns << 8) & (!other_pieces)
        }
    }
    fn pawn_double_pushes(&self) -> u64 {
        if self.active_player == Player::Black {
            let pawns = self.black.pawns & 0x00FF000000000000;
            let enemy_pieces = self.all_pieces() & !pawns;
            (pawns >> 16) & (!enemy_pieces) & (!(enemy_pieces >> 8))
        }
        else {
            let pawns = self.white.pawns & 0x000000000000FF00;
            let enemy_pieces = self.all_pieces() & !pawns;
            (pawns << 16) & (!enemy_pieces) & (!(enemy_pieces << 8))
        }
    }
}