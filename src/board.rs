#[cfg(test)]
mod test {
    
    use super::*;
    #[test]
    fn king_moves_empty_board() {
        let board = BoardState {
            white:PlayerState{king:0x0000000800000000, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            black: PlayerState{king:0, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0
        };
        let king_move = board.king_moves();
        assert_eq!(king_move, 0x00001C141C000000);
        let board = BoardState {
            white:PlayerState{king:0x0000000100000000, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            black: PlayerState{king:0, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0
        };
        let king_move = board.king_moves();
        assert_eq!(king_move, 0x0000030203000000);
        let board = BoardState {
            white:PlayerState{king:0x0000008000000000, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            black: PlayerState{king:0, queens: 0, rooks: 0, pawns: 0, bishops: 0, knights: 0, queen_castle: false, king_castle: false},
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0
        };
        let king_move = board.king_moves();
        assert_eq!(king_move, 0x0000C040C0000000);
        //for i in 0..=7u8 {
        //    let row = (king_move >> (56 - (i * 8))) as u8;
        //    println!("{:08b}", row);
        //}
    }
    #[test]
    fn king_moves_starting_board() {
        let board = BoardState{
            black: PlayerState { 
                king: 0x0800000000000000, 
                queens: 0x1000000000000000, 
                bishops: 0x2400000000000000, 
                knights: 0x4200000000000000, 
                rooks: 0x8100000000000000, 
                pawns: 0x00FF000000000000, 
                king_castle: true, 
                queen_castle: true
            },
            white: PlayerState {
                king: 0x08,
                queens: 0x10,
                bishops: 0x24,
                knights: 0x42,
                rooks: 0x81,
                pawns: 0xFF00,
                king_castle: true, 
                queen_castle: true
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            half_counter: 0,
            full_counter: 1
        };
        assert_eq!(board.king_moves(), 0x0);
    }
    #[test]
    fn king_moves_bongcloud() {
        let board = BoardState{
            black: PlayerState {
                king: 0x0800000000000000,
                queens: 0x1000000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                rooks: 0x8100000000000000,
                pawns: 0x00F7080000000000,
                king_castle: true, 
                queen_castle: true
            },
            white: PlayerState {
                king: 0x08,
                queens: 0x10,
                bishops: 0x24,
                knights: 0x42,
                rooks: 0x81,
                pawns: 0x0800F700,
                king_castle: true, 
                queen_castle: true
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x13),
            half_counter: 0,
            full_counter: 2
        };
        let king_move = board.king_moves();
        assert_eq!(board.king_moves(), 0x0000000000000800);
    }
}

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
            let other_pieces = self.all_pieces() & !pawns;
            (pawns >> 16) & (!other_pieces) & (!(other_pieces >> 8))
        }
        else {
            let pawns = self.white.pawns & 0x000000000000FF00;
            let other_pieces = self.all_pieces() & !pawns;
            (pawns << 16) & (!other_pieces) & (!(other_pieces << 8))
        }
    }
///attacks are stored in an array [west_attacks, east_attacks]
    fn pawn_attacks(&self) -> [u64;2] {
        let not_a_file = 0x7F7F7F7F7F7F7F7Fu64;
        let not_h_file = 0xFEFEFEFEFEFEFEFEu64;
        if self.active_player == Player::Black {
            let enemy_pieces = self.white.all_pieces();
            let pawns = self.black.pawns;
            let west_attack_squares = (pawns >> 9) & not_h_file;
            let east_attack_squares = (pawns >> 7) & not_a_file;
            [west_attack_squares & (enemy_pieces | self.en_passant_target.targeted_square()),
            east_attack_squares & (enemy_pieces | self.en_passant_target.targeted_square())]
        }
        else {
            let enemy_pieces = self.black.all_pieces();
            let pawns = self.white.pawns;
            let west_attack_squares = (pawns >> 7) & not_h_file;
            let east_attack_squares = (pawns >> 9) & not_a_file;
            [west_attack_squares & (enemy_pieces | self.en_passant_target.targeted_square()),
            east_attack_squares & (enemy_pieces | self.en_passant_target.targeted_square())]
        }
    }

    fn king_moves(&self) -> u64 {
        let not_a_file = 0x7F7F7F7F7F7F7F7Fu64;
        let not_h_file = 0xFEFEFEFEFEFEFEFEu64;
        let player = if self.active_player == Player::Black {&self.black} else {&self.white};
        let king = player.king;
        let other_pieces = player.all_pieces(); //can ignore removing king from board because it cannot move to the same square
        (((king << 9 | king << 1 | king >> 7) & not_h_file) | king << 8 | ((king << 7  | king >> 1 | king >> 9) & not_a_file) |  king >> 8) & !other_pieces
    }
}