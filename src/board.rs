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
        let queen_move = queen_moves(
            board.white.queens.ilog2() as usize,
            board.all_pieces(),
            board.white.all_pieces(),
        );
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

    #[test]
    fn first_turn_legal_moves() {
        let mut move_array = [Move { from: 0, to: 0 }; 218];
        let board = BoardState {
            black: PlayerState {
                king: 0x0800000000000000,
                queens: 0x1000000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                rooks: 0x8100000000000000,
                pawns: 0x00FF000000000000,
                king_castle: true,
                queen_castle: true,
            },
            white: PlayerState {
                king: 0x08,
                queens: 0x10,
                bishops: 0x24,
                knights: 0x42,
                rooks: 0x81,
                pawns: 0xFF00,
                king_castle: true,
                queen_castle: true,
            },
            active_player: Player::White,
            en_passant_target: EnPassantTarget(0x80),
            half_counter: 0,
            full_counter: 1,
        };
        let moves = board.generate_moves(&mut move_array);
        let x = moves.len();
        dbg!(moves_to_algebraic(moves, &board));
        assert_eq!(x, 20);
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
        1u64 << (self.0 & 0b00111111) * (self.0 >> 7)
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

#[derive(Debug, Clone, Copy)]
struct Move {
    from: u8,
    to: u8,
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
    ///move_list is treated as a stack, and a slice of it containing the number of valid moves
    /// is returned by this function. the rest of move_list should be assumed to be invalid.
    fn generate_moves<'a>(&self, move_list: &'a mut [Move; 218]) -> &'a mut [Move] {
        let mut move_index = 0usize;
        let (player, opponent) = if self.active_player == Player::Black {
            (&self.black, &self.white)
        } else {
            (&self.white, &self.black)
        };
        let all_pieces = self.all_pieces();
        let king_square = player.king.ilog2() as usize;
        let attacked_squares: u64 = find_opponent_attacked_squares(
            all_pieces,
            opponent,
            &self.en_passant_target,
            self.active_player,
        );
        let mut king_move = king_moves(king_square, player.all_pieces()) & !attacked_squares;
        let king_moves = indices_from_bitboard(king_move);
        for move_ in king_moves {
            move_list[move_index] = Move {
                from: king_square as u8,
                to: move_,
            };
            move_index += 1;
        }
        let checked_by = pieces_checked_by(
            all_pieces,
            player,
            opponent,
            &self.en_passant_target,
            self.active_player,
        );
        let mut capture_mask = 0xFFFFFFFFFFFFFFFFu64;
        let mut push_mask = 0xFFFFFFFFFFFFFFFFu64;

        match checked_by.count_ones() {
            0 => {}
            1 => {
                capture_mask = checked_by;
                if (opponent.bishops | opponent.rooks | opponent.queens) & checked_by != 0 {
                    push_mask = queen_moves(king_square, all_pieces, all_pieces)
                        & queen_moves(checked_by.ilog2() as usize, all_pieces, all_pieces)
                } else {
                    push_mask = 0
                };
            }
            _ => return move_list.as_mut_slice().split_at_mut(move_index).0,
        }
        let pinned_pieces = king_pins_bishop(
            king_square,
            player.all_pieces(),
            all_pieces,
            opponent.bishops | opponent.queens,
        ) | king_pins_rook(
            king_square,
            player.all_pieces(),
            all_pieces,
            opponent.rooks | opponent.queens,
        );
        for rook in indices_from_bitboard(player.rooks) {
            let mut moves = rook_moves(rook as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << rook & pinned_pieces != 0 {
                moves = moves & queen_moves(king_square, 0, 0);
            }
            insert_moves(rook, moves, move_list, &mut move_index);
        }
        for bishop in indices_from_bitboard(player.bishops) {
            let mut moves = bishop_moves(bishop as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << bishop & pinned_pieces != 0 {
                moves = moves & queen_moves(king_square, 0, 0);
            }
            insert_moves(bishop, moves, move_list, &mut move_index);
        }
        for knight in indices_from_bitboard(player.knights) {
            let mut moves =
                knight_moves(knight as usize, player.all_pieces()) & (capture_mask | push_mask);
            if 1 << knight & pinned_pieces != 0 {
                moves = 0;
            } else {
                insert_moves(knight, moves, move_list, &mut move_index);
            }
        }
        for queen in indices_from_bitboard(player.queens) {
            let mut moves = queen_moves(queen as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << queen & pinned_pieces != 0 {
                moves &= queen_moves(king_square, 0, 0);
            }
            insert_moves(queen, moves, move_list, &mut move_index);
        }
        //does not properly handle checks from en passant moves, will fix later
        for pawn in indices_from_bitboard(player.pawns) {
            let attacks = pawn_attacks(
                1 << pawn,
                opponent.all_pieces(),
                self.en_passant_target.clone(),
                self.active_player,
            );
            let mut moves = pawn_single_pushes(1 << pawn, all_pieces, self.active_player)
                | pawn_double_pushes(1 << pawn, all_pieces)
                | attacks[0]
                | attacks[1];
            if 1 << pawn & pinned_pieces != 0 {
                moves &= queen_moves(king_square, 0, 0);
            }
            insert_moves(pawn, moves, move_list, &mut move_index);
        }
        move_list.as_mut_slice().split_at_mut(move_index).0
    }
}

fn indices_from_bitboard(board: u64) -> Vec<u8> {
    let mut result = Vec::with_capacity(board.count_ones() as usize);
    let mut index = board.trailing_zeros() as u8;
    let mut new_board = board;
    while index <= 63 && new_board != 0 {
        new_board = (board >> index) & !1;
        result.push(index);
        index += new_board.trailing_zeros() as u8;
    }
    result
}

fn find_opponent_attacked_squares(
    all_pieces: u64,
    opponent: &PlayerState,
    en_passant_target: &EnPassantTarget,
    active_player: Player,
) -> u64 {
    let pawn_attacked_squares = pawn_attacks(
        opponent.pawns,
        !0, //leads to all possible pawn attacks being marked
        en_passant_target.clone(),
        active_player,
    );
    let mut attacked_squares = 0;
    attacked_squares |= indices_from_bitboard(opponent.rooks)
        .iter()
        .fold(0, |x, y| x | rook_moves(*y as usize, all_pieces, 0));
    attacked_squares |= indices_from_bitboard(opponent.bishops)
        .iter()
        .fold(0, |x, y| x | bishop_moves(*y as usize, all_pieces, 0));
    attacked_squares |= indices_from_bitboard(opponent.queens)
        .iter()
        .fold(0, |x, y| x | queen_moves(*y as usize, all_pieces, 0));
    attacked_squares |= indices_from_bitboard(opponent.knights)
        .iter()
        .fold(0, |x, y| x | knight_moves(*y as usize, 0));
    attacked_squares |= pawn_attacked_squares[0] | pawn_attacked_squares[1];
    attacked_squares
}

fn pieces_checked_by(
    all_pieces: u64,
    player: &PlayerState,
    opponent: &PlayerState,
    en_passant_target: &EnPassantTarget,
    active_player: Player,
) -> u64 {
    let pawn_attacked_squares = pawn_attacks(
        opponent.pawns,
        !0, //leads to all possible pawn attacks being marked
        en_passant_target.clone(),
        active_player,
    );
    let pawns_attacking_king = if active_player == Player::Black {
        (pawn_attacked_squares[0] & player.king) >> 9
            | (pawn_attacked_squares[1] & player.king) >> 7
    } else {
        (pawn_attacked_squares[0] & player.king) << 7
            | (pawn_attacked_squares[1] & player.king) << 9
    };
    king_attacked_rooks(
        player.king.ilog2() as usize,
        all_pieces,
        opponent.rooks | opponent.queens,
    ) | king_attacked_bishops(
        player.king.ilog2() as usize,
        all_pieces,
        opponent.bishops | opponent.queens,
    ) | king_attacked_knights(player.king.ilog2() as usize, opponent.knights)
        | pawns_attacking_king
}

fn insert_moves(piece: u8, moves: u64, move_list: &mut [Move; 218], move_index: &mut usize) {
    let move_squares = indices_from_bitboard(moves);
    for move_ in move_squares {
        move_list[*move_index] = Move {
            from: piece,
            to: move_,
        };
        *move_index += 1;
    }
}

///Pawns are able to push forward when there is no piece blocking their way
fn pawn_single_pushes(pawns: u64, other_pieces: u64, player: Player) -> u64 {
    if player == Player::Black {
        (pawns >> 8) & (!other_pieces)
    } else {
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
    } else {
        0
    }
}
///attacks are stored in an array [west_attacks, east_attacks]
fn pawn_attacks(
    pawns: u64,
    enemy_pieces: u64,
    en_passant_target: EnPassantTarget,
    player: Player,
) -> [u64; 2] {
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
        let west_attack_squares = (pawns << 7) & not_h_file;
        let east_attack_squares = (pawns << 9) & not_a_file;
        [
            west_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
            east_attack_squares & (enemy_pieces | en_passant_target.targeted_square()),
        ]
    }
}

fn king_moves(square: usize, friendly_pieces: u64) -> u64 {
    crate::precomputed::KING_MOVES[square] & !friendly_pieces
}

fn king_attacked_rooks(king: usize, all_pieces: u64, rooks_and_queens: u64) -> u64 {
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
            if mask != 0 && masked_pieces.count_ones() == 0 {
                pinned_pieces |= 1 << index;
            }
        }
        index += new_rooks_and_queens.trailing_zeros();
    }
    pinned_pieces
}

fn king_attacked_bishops(king: usize, all_pieces: u64, bishops_and_queens: u64) -> u64 {
    let mut new_bishops_and_queens = bishops_and_queens;
    let mut index = bishops_and_queens.trailing_zeros();
    let king_moves = bishop_moves(king, bishops_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_bishops_and_queens != 0 {
        new_bishops_and_queens = (bishops_and_queens >> index) & !1;
        let moves = bishop_moves(index as usize, 1 << king, 0);
        if (moves & king_moves).count_ones() > 1 && (moves & king_moves) & (1 << king) != 0 {
            let mask = moves & king_moves;
            let masked_pieces = (all_pieces & !(1 << king) as u64) & mask;
            if mask != 0 && masked_pieces.count_ones() == 0 {
                pinned_pieces |= 1 << index;
            }
        }
        index += new_bishops_and_queens.trailing_zeros();
    }
    pinned_pieces
}

fn king_attacked_knights(king: usize, enemy_knights: u64) -> u64 {
    knight_moves(king, 0) & enemy_knights
}

///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_rook(
    king: usize,
    friendly_pieces: u64,
    all_pieces: u64,
    rooks_and_queens: u64,
) -> u64 {
    let mut new_rooks_and_queens = rooks_and_queens;
    let mut index = rooks_and_queens.trailing_zeros();
    let king_moves = rook_moves(king, rooks_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_rooks_and_queens != 0 {
        new_rooks_and_queens = (rooks_and_queens >> index) & !1;
        let moves = rook_moves(index as usize, 1 << king, 0);
        if (moves & king_moves).count_ones() > 1 && (moves & king_moves) & (1 << king) != 0 {
            let mask = moves & king_moves;
            let masked_pieces = (all_pieces & !(1 << king) as u64) & mask;
            if masked_pieces.count_ones() == 1
                && (masked_pieces & friendly_pieces).count_ones() == 1
            {
                pinned_pieces |= masked_pieces;
            }
        }
        index += new_rooks_and_queens.trailing_zeros();
    }
    pinned_pieces
}

///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_bishop(
    king: usize,
    friendly_pieces: u64,
    all_pieces: u64,
    bishops_and_queens: u64,
) -> u64 {
    let mut new_bishops_and_queens = bishops_and_queens;
    let mut index = bishops_and_queens.trailing_zeros();
    let king_moves = bishop_moves(king, bishops_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_bishops_and_queens != 0 {
        new_bishops_and_queens = (bishops_and_queens >> index) & !1;
        let moves = bishop_moves(index as usize, 1 << king, 0);
        if (moves & king_moves).count_ones() > 1 && (moves & king_moves) & (1 << king) != 0 {
            let mask = moves & king_moves;
            let masked_pieces = (all_pieces & !(1 << king) as u64) & mask;
            if masked_pieces.count_ones() == 1
                && (masked_pieces & friendly_pieces).count_ones() == 1
            {
                pinned_pieces |= masked_pieces;
            }
        }
        index += new_bishops_and_queens.trailing_zeros();
    }
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

fn moves_to_algebraic(moves: &[Move], board: &BoardState) -> Vec<String> {
    //does not account for captures, check, or checkmate yet
    let mut result = vec![];
    static squares: [&'static str; 64] = [
        "h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1", "h2", "g2", "f2", "e2", "d2", "c2", "b2",
        "a2", "h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3", "h4", "g4", "f4", "e4", "d4", "c4",
        "b4", "a4", "h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5", "h6", "g6", "f6", "e6", "d6",
        "c6", "b6", "a6", "h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7", "h8", "g8", "f8", "e8",
        "d8", "c8", "b8", "a8",
    ];
    for m in moves {
        let to_square = squares[m.to as usize];
        let piece_square = 1u64 << m.from;
        let piece = if piece_square & board.black.rooks != 0 {
            "r"
        } else if piece_square & board.black.bishops != 0 {
            "b"
        } else if piece_square & board.black.knights != 0 {
            "n"
        } else if piece_square & board.black.queens != 0 {
            "q"
        } else if piece_square & board.black.king != 0 {
            "k"
        } else if piece_square & board.white.rooks != 0 {
            "R"
        } else if piece_square & board.white.bishops != 0 {
            "B"
        } else if piece_square & board.white.knights != 0 {
            "N"
        } else if piece_square & board.white.queens != 0 {
            "Q"
        } else if piece_square & board.white.king != 0 {
            "K"
        } else {
            ""
        };
        result.push(piece.to_owned() + to_square);
    }
    result
}

fn queen_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let bishop_magic = crate::precomputed::bishop_magic::BISHOP_MAGICS[square];
    let bishop_blockers = all_pieces & crate::precomputed::BISHOP_MASK[square];
    let bishop_magic_index = crate::precomputed::magic_to_index(bishop_magic, bishop_blockers, 9);
    let rook_magic = crate::precomputed::rook_magic::ROOK_MAGICS[square];
    let rook_blockers = all_pieces & crate::precomputed::ROOK_MASK[square];
    let rook_magic_index = crate::precomputed::magic_to_index(rook_magic, rook_blockers, 12);
    (crate::precomputed::bishop_magic::BISHOP_ATTACKS[square][bishop_magic_index]
        | crate::precomputed::rook_magic::ROOK_ATTACKS[square][rook_magic_index])
        & !friendly_pieces
}
