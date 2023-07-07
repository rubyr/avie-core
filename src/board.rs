use crate::gamestate::{File, ParsedGameState, Player, Rank};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn king_moves_empty_board() {
        let board = BoardState {
            white: PlayerState {
                king: 0x0000000800000000,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            black: PlayerState {
                king: 0,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0,
        };
        let king_move = king_moves(board.white.king.ilog2() as usize, board.white.all_pieces());
        assert_eq!(king_move, 0x00001C141C000000);
        let board = BoardState {
            white: PlayerState {
                king: 0x0000000100000000,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            black: PlayerState {
                king: 0,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0,
        };
        let king_move = king_moves(board.white.king.ilog2() as usize, board.white.all_pieces());
        assert_eq!(king_move, 0x0000030203000000);
        let board = BoardState {
            white: PlayerState {
                king: 0x0000008000000000,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            black: PlayerState {
                king: 0,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0,
        };
        let king_move = king_moves(board.white.king.ilog2() as usize, board.white.all_pieces());
        assert_eq!(king_move, 0x0000C040C0000000);
        //for i in 0..=7u8 {
        //    let row = (king_move >> (56 - (i * 8))) as u8;
        //    println!("{:08b}", row);
        //}
    }

    #[test]
    fn king_moves_bongcloud() {
        let board = BoardState {
            black: PlayerState {
                king: 0x0800000000000000,
                queens: 0x1000000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                rooks: 0x8100000000000000,
                pawns: 0x00F7080000000000,
                king_castle: true,
                queen_castle: true,
            },
            white: PlayerState {
                king: 0x08,
                queens: 0x10,
                bishops: 0x24,
                knights: 0x42,
                rooks: 0x81,
                pawns: 0x0800F700,
                king_castle: true,
                queen_castle: true,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x13),
            half_counter: 0,
            full_counter: 2,
        };
        let king_move = king_moves(board.white.king.ilog2() as usize, board.white.all_pieces());
        assert_eq!(king_move, 0x0000000000000800);
    }

    #[test] 
    fn knight_moves_empty_board() {
        let board = BoardState {
            white: PlayerState {
                king: 0,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0x8000000008000000,
                queen_castle: false,
                king_castle: false,
            },
            black: PlayerState {
                king: 0,
                queens: 0,
                rooks: 0,
                pawns: 0,
                bishops: 0,
                knights: 0,
                queen_castle: false,
                king_castle: false,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            full_counter: 1,
            half_counter: 0,
        };
        let mut new_knights = board.white.knights;
        let mut index = new_knights.trailing_zeros();
        let friendly_pieces = board.white.all_pieces();
        let mut moves = vec![];
        while index <= 63 && new_knights != 0 {
            new_knights = (new_knights >> index) & !1;
            moves.push(knight_moves(index as usize, friendly_pieces));
            index += new_knights.trailing_zeros();
        }
        assert_eq!(moves, vec![0x0000142200221400u64, 0x0020400000000000u64]);
    }

    #[test]
    fn queen_moves_empty_board() {
        let board = BoardState {
            black: PlayerState {
                king: 0x0800000000000000,
                queens: 0x1000000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                rooks: 0x8100000000000000,
                pawns: 0x00F7080000000000,
                king_castle: true,
                queen_castle: true,
            },
            white: PlayerState {
                king: 0x08,
                queens: 0x10,
                bishops: 0x24,
                knights: 0x42,
                rooks: 0x81,
                pawns: 0x0800F700,
                king_castle: true,
                queen_castle: true,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x13),
            half_counter: 0,
            full_counter: 2,
        };
        let queen_move = queen_moves(board.white.queens.ilog2() as usize, board.all_pieces(), board.white.all_pieces());
        assert_eq!(queen_move, 0x0000000102040800);
    }

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
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ],
            active_player: Player::White,
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true,
            },
            en_passant_target: None,
            half_turn_clock: 0,
            full_turn_clock: 1,
        });
        assert_eq!(
            result,
            BoardState {
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
            }
        );
        let result = BoardState::new(ParsedGameState {
            piece_position: [
                ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', 'P', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['P', 'P', 'P', 'P', '.', 'P', 'P', 'P'],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ],
            active_player: Player::Black,
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true,
            },
            en_passant_target: Some((File::E, Rank::Three)),
            half_turn_clock: 0,
            full_turn_clock: 1,
        });
        assert_eq!(
            result,
            BoardState {
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
            }
        );
    }

    #[test]
    fn king_rook_pins() {
        let king = 0x0000000008000000u64;
        let rooks = 0x0800000081000000u64;
        let enemy = 0x08000000C1000000u64;
        let friendly = 0x000800001C000000u64;
        let result = king_pins_rook(king.ilog2() as usize, friendly, enemy | friendly, rooks);
        assert_eq!(result, 0x0008000004000000);
    }
    /// 10000000
    /// 00000001
    /// 00000010
    /// 00010100
    /// 00001000
    /// 00000000
    /// 00000000
    /// 00000000
    #[test]
    fn king_bishop_pins() {
        let king = 0x0000000008000000u64;
        let bishops = 0x8001000000000001u64;
        let enemy = 0x8001020000000001u64;
        let friendly = 0x0000001408000200u64;
        let result = king_pins_bishop(king.ilog2() as usize, friendly, enemy | friendly, bishops);
        assert_eq!(result, 0x0000001000000200);
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

fn player_from_gamestate(player: Player, gamestate: &ParsedGameState) -> PlayerState {
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
    PlayerState {
        rooks,
        bishops,
        knights,
        queens,
        king,
        pawns,
        king_castle,
        queen_castle,
    }
}

fn en_passant_target(target: &Option<(File, Rank)>) -> EnPassantTarget {
    match target {
        None => EnPassantTarget(0x80),
        Some((file, rank)) => {
            let playermask = if *rank == Rank::Six {
                0b01000000u8
            } else {
                0u8
            };
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
        let result = Self {
            black,
            white,
            en_passant_target,
            active_player: game.active_player,
            half_counter: game.half_turn_clock,
            full_counter: game.full_turn_clock,
        };
        debug_assert_eq!(result.is_valid(), true);
        result
    }

    fn is_valid(&self) -> bool {
        self.white.is_valid() & self.black.is_valid()
    }

    fn all_pieces(&self) -> u64 {
        self.black.all_pieces() | self.white.all_pieces()
    }
    
}

///Pawns are able to push forward when there is no piece blocking their way
fn pawn_single_pushes(pawns: u64, other_pieces: u64, player: Player) -> u64 {
    if player == Player::Black {
        (pawns >> 8) & (!other_pieces)
    }
    else {
        (pawns << 8) & (!other_pieces)
    }
}
fn pawn_double_pushes(pawns: u64, other_pieces: u64) -> u64 {
        if pawns & 0x00FF000000000000 != 0 {
            let other_pieces = other_pieces & !pawns;
            ((pawns & 0x00FF000000000000) >> 16) & (!other_pieces) & (!(other_pieces >> 8))
        } else if pawns & 0x000000000000FF00 != 0 {
            let other_pieces = other_pieces & !pawns;
            ((pawns & 0x000000000000FF00) << 16) & (!other_pieces) & (!(other_pieces << 8))
        }
        else {0}
}
///attacks are stored in an array [west_attacks, east_attacks]
fn pawn_attacks(pawns: u64, enemy_pieces: u64, en_passant_target: EnPassantTarget, player: Player) -> [u64; 2] {
    let not_a_file = 0x7F7F7F7F7F7F7F7Fu64;
    let not_h_file = 0xFEFEFEFEFEFEFEFEu64;
    if player == Player::Black {
        let west_attack_squares = (pawns >> 9) & not_h_file;
        let east_attack_squares = (pawns >> 7) & not_a_file;
        [
            west_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
            east_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
        ]
    } else {
        let west_attack_squares = (pawns >> 7) & not_h_file;
        let east_attack_squares = (pawns >> 9) & not_a_file;
        [
            west_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
            east_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
        ]
    }
}

fn king_moves(square: usize, friendly_pieces: u64) -> u64 {
    crate::precomputed::KING_MOVES[square] & !friendly_pieces
}

fn king_moves_legal(square: usize, friendly_pieces: u64, enemy_attacks: u64) -> u64 {
    let moves = king_moves(square, friendly_pieces);
    moves & !enemy_attacks
}

///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_rook(king: usize, friendly_pieces: u64, all_pieces: u64, rooks_and_queens: u64) -> u64 {
    let mut new_rooks_and_queens = rooks_and_queens;
    let mut index = rooks_and_queens.trailing_zeros();
    let king_moves = rook_moves(king, rooks_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_rooks_and_queens != 0 {
            new_rooks_and_queens = (rooks_and_queens >> index) & !1;
            let moves = rook_moves(index as usize, 1 << king, 0);
            if (moves & king_moves).count_ones() > 1 {
                let mask = moves & king_moves;
                let masked_pieces = (all_pieces & !(1 << king) as u64) & mask;
                if masked_pieces.count_ones() == 1 && (masked_pieces & friendly_pieces).count_ones() == 1 {
                    pinned_pieces |= masked_pieces;
                }
            }
            index += new_rooks_and_queens.trailing_zeros();
    };
    pinned_pieces
}


///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_bishop(king: usize, friendly_pieces: u64, all_pieces: u64, bishops_and_queens: u64) -> u64 {
    let mut new_bishops_and_queens = bishops_and_queens;
    let mut index = bishops_and_queens.trailing_zeros();
    let king_moves = bishop_moves(king, bishops_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_bishops_and_queens != 0 {
            new_bishops_and_queens = (bishops_and_queens >> index) & !1;
            let moves = bishop_moves(index as usize, 1 << king, 0);
            if (moves & king_moves).count_ones() > 1 {
                let mask = moves & king_moves;
                let masked_pieces = (all_pieces & !(1 << king) as u64) & mask;
                if masked_pieces.count_ones() == 1 && (masked_pieces & friendly_pieces).count_ones() == 1 {
                    pinned_pieces |= masked_pieces;
                }
            }
            index += new_bishops_and_queens.trailing_zeros();
    };
    pinned_pieces
}

fn knight_moves(square: usize, friendly_pieces: u64) -> u64 {
    crate::precomputed::KNIGHT_MOVES[square] & !friendly_pieces
}

fn bishop_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
        let magic = crate::precomputed::bishop_magic::BISHOP_MAGICS[square];
        let blockers = all_pieces & crate::precomputed::BISHOP_MASK[square];
        let magic_index = crate::precomputed::magic_to_index(magic, blockers, 9);
        crate::precomputed::bishop_magic::BISHOP_ATTACKS[square][magic_index] & !friendly_pieces
}

fn rook_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let magic = crate::precomputed::rook_magic::ROOK_MAGICS[square];
    let blockers = all_pieces & crate::precomputed::ROOK_MASK[square];
    let magic_index = crate::precomputed::magic_to_index(magic, blockers, 12);
    crate::precomputed::rook_magic::ROOK_ATTACKS[square][magic_index] & !friendly_pieces
}

fn queen_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let bishop_magic = crate::precomputed::bishop_magic::BISHOP_MAGICS[square];
    let bishop_blockers = all_pieces & crate::precomputed::BISHOP_MASK[square];
    let bishop_magic_index = crate::precomputed::magic_to_index(bishop_magic, bishop_blockers, 9);
    let rook_magic = crate::precomputed::rook_magic::ROOK_MAGICS[square];
    let rook_blockers = all_pieces & crate::precomputed::ROOK_MASK[square];
    let rook_magic_index = crate::precomputed::magic_to_index(rook_magic, rook_blockers, 12);
    (crate::precomputed::bishop_magic::BISHOP_ATTACKS[square][bishop_magic_index] | crate::precomputed::rook_magic::ROOK_ATTACKS[square][rook_magic_index]) & !friendly_pieces
}
