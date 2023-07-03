use crate::gamestate::{ParsedGameState, Player, File, Rank};
#[cfg(test)]
mod test {
    use super::*;
    use crate::gamestate::*;
    #[test]
    fn boardstate_new() {
        let result = BoardState::new(ParsedGameState {
            piece_position: [
                ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                ['p'; 8],
                ['.'; 8],
                ['.'; 8],
                ['.'; 8],
                ['.'; 8],
                ['P'; 8],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
            ],
            active_player: Player::White,
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true
            },
            en_passant_target: None,
            half_turn_clock: 0,
            full_turn_clock: 1
        });
        assert_eq!(result, BoardState{
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
        });
        let result = BoardState::new(ParsedGameState {
            piece_position: [
                ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', 'P', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['P', 'P', 'P', 'P', '.', 'P', 'P', 'P'],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
            ],
            active_player: Player::Black,
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true
            },
            en_passant_target: Some((File::E, Rank::Three)),
            half_turn_clock: 0,
            full_turn_clock: 1
        });
        assert_eq!(result, BoardState{
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
                pawns: 0x0800F700,
                king_castle: true, 
                queen_castle: true
            },
            active_player: Player::Black,
            en_passant_target: EnPassantTarget(0x13),
            half_counter: 0,
            full_counter: 1
        });
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PlayerState {
    king: u64,
    queens: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    pawns: u64,
    king_castle: bool,
    queen_castle: bool,
}

impl PlayerState {
    ///function uses a quirk of binary representation to verify that there are no duplicate pieces on the board.
    /// when no bits are shared between two numbers, addition gives the same result as binary OR. by adding all
    /// bitboards together and comparing with all bitboards OR'd together, we can ensure that a player's state is valid.
    fn is_valid(&self) -> bool {
        let mut valid = self.king.count_ones() == 1;
        let add_board: u128 = self.king as u128
            + self.queens as u128
            + self.bishops as u128
            + self.knights as u128
            + self.rooks as u128
            + self.pawns as u128;
        valid &= add_board == self.all_pieces() as u128;
        valid
    }
    fn all_pieces(&self) -> u64 {
        self.king | self.queens | self.bishops | self.knights | self.rooks | self.pawns
    }
}

///En Passant Target representation:
///  0bX0000000: active flag. if 1, there was no en passant on the previous turn, and all other bits are ignored.
///  0b0X000000: player flag. 0 for white, 1 for black.
///  0b00XXXXXX: square of valid en passant target. bitboard is obtained by shifting 1u64 by this value.
#[derive(Clone, PartialEq, Eq, Debug)]
struct EnPassantTarget(u8);

impl EnPassantTarget {
    fn targeted_player(&self) -> Option<Player> {
        match self.0 >> 6 {
            0 => Some(Player::White),
            1 => Some(Player::Black),
            2 | 3 => None,
            _ => unreachable!(),
        }
    }
    fn targeted_square(&self) -> u64 {
        1u64 << self.0 & 0b00111111
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
    black: PlayerState,
    white: PlayerState,
    en_passant_target: EnPassantTarget,
    active_player: Player,
    half_counter: u8,
    full_counter: u32, //pretty sure it's impossible to have a chess game go longer than 4 billion moves, but we'll see
}

fn into_bitboard(matched_char: char, board: &[[char; 8]; 8]) -> u64 {
    let board: &[char; 64] = unsafe { std::mem::transmute(board) };
    let bool_board = board.map(|x| x == matched_char);
    bool_board
        .iter()
        .fold(0u64, |result, &x| (result << 1) ^ x as u64)
}

fn player_from_gamestate(
    player: Player,
    gamestate: &ParsedGameState
) -> PlayerState {
    let board = &gamestate.piece_position;
    let (chars, king_castle, queen_castle) = if player == Player::Black {
        (
            ['r', 'b', 'n', 'q', 'k', 'p'],
            gamestate.castling_rights.black_kingside,
            gamestate.castling_rights.black_queenside,
        )
    } else {
        (
            ['R', 'B', 'N', 'Q', 'K', 'P'],
            gamestate.castling_rights.white_kingside,
            gamestate.castling_rights.white_queenside,
        )
    };
    let rooks = into_bitboard(chars[0], board);
    let bishops = into_bitboard(chars[1], board);
    let knights = into_bitboard(chars[2], board);
    let queens = into_bitboard(chars[3], board);
    let king = into_bitboard(chars[4], board);
    let pawns = into_bitboard(chars[5], board);
    PlayerState{rooks, bishops, knights, queens, king, pawns, king_castle, queen_castle}
}

fn en_passant_target(target: &Option<(File, Rank)>) -> EnPassantTarget {
    match target {
        None => EnPassantTarget(0x80),
        Some((file, rank)) => {
            let playermask= if *rank == Rank::Six {0b01000000u8} else {0u8};
            let square: u8 = (7 - *file as u8) + (*rank as u8 * 8);
            EnPassantTarget(playermask | square)
        }
    }
}

impl BoardState {
    fn new(game: ParsedGameState) -> Self {
        let black = player_from_gamestate(Player::Black, &game);
        let white = player_from_gamestate(Player::White, &game);
        let en_passant_target = en_passant_target(&game.en_passant_target);
        let result = Self{black, white, en_passant_target, active_player: game.active_player, half_counter: game.half_turn_clock, full_counter: game.full_turn_clock};
        debug_assert_eq!(result.is_valid(), true);
        result 
    }

    fn is_valid(&self) -> bool{
        self.white.is_valid() & self.black.is_valid()
    }

    fn all_pieces(&self) -> u64 {
        self.black.all_pieces() | self.white.all_pieces()
    }
}
