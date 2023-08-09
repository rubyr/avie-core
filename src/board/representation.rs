use crate::gamestate::{File, ParsedGameState, Player, Rank};
use crate::board::*;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayerState {
    pub king: u64,
    pub queens: u64,
    pub bishops: u64,
    pub knights: u64,
    pub rooks: u64,
    pub pawns: u64,
    pub king_castle: bool,
    pub queen_castle: bool,
}

impl PlayerState {
    ///function uses a quirk of binary representation to verify that there are no duplicate pieces on the board.
    /// when no bits are shared between two numbers, addition gives the same result as binary OR. by adding all
    /// bitboards together and comparing with all bitboards OR'd together, we can ensure that a player's state is valid.
    pub fn is_valid(&self) -> bool {
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
    pub fn all_pieces(&self) -> u64 {
        self.king | self.queens | self.bishops | self.knights | self.rooks | self.pawns
    }
}

///En Passant Target representation:
///  0bX0000000: active flag. if 1, there was no en passant on the previous turn, and all other bits are ignored.
///  0b0X000000: player flag. 0 for white, 1 for black.
///  0b00XXXXXX: square of valid en passant target. bitboard is obtained by shifting 1u64 by this value.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct EnPassantTarget(pub u8);
pub static EN_PASSANT_NO_SQUARE: u8 = 0b10000000;
pub static EN_PASSANT_SQUARE_MASK: u8 = 0b00111111;

impl EnPassantTarget {
    pub fn targeted_player(&self) -> Option<Player> {
        match self.0 >> 6 {
            0 => Some(Player::White),
            1 => Some(Player::Black),
            //all other values mean there is no en passant target
            _ => None,
        }
    }
    pub fn targeted_square(&self) -> u64 {
        if self.0 & EN_PASSANT_NO_SQUARE != 0 {
            return 0;
        }
        1u64 << (self.0 & EN_PASSANT_SQUARE_MASK)
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CapturedPiece {
    None,
    Pawn(u8),
    Knight(u8),
    Bishop(u8),
    Rook(u8),
    Queen(u8),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnmakeMoveData {
    pub castling_rights: crate::gamestate::CastlingRights,
    pub capture: CapturedPiece,
    pub en_passant: EnPassantTarget,
    pub halfmove_clock: u8,
}

pub(crate) fn into_bitboard(matched_char: char, board: &[[char; 8]; 8]) -> u64 {
    let board: &[char; 64] = unsafe { std::mem::transmute(board) };
    let bool_board = board.map(|x| x == matched_char);
    bool_board
        .iter()
        .fold(0u64, |result, &x| (result << 1) ^ x as u64)
}

pub(crate) fn player_from_gamestate(player: Player, gamestate: &ParsedGameState) -> PlayerState {
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Promotion {
    None,
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Promotion, //0: no promotion, 1: knight, 2: bishop, 3: rook, 4: queen
}

impl Move {
    pub fn new(from: u8, to: u8, promotion: Promotion) -> Move {
        Move {
            from,
            to,
            promotion,
        }
    }
}

impl Default for Move {
    fn default() -> Move {
        Move::new(0, 0, Promotion::None)
    }
}

pub fn en_passant_target(target: &Option<(File, Rank)>) -> EnPassantTarget {
    match target {
        None => EnPassantTarget(EN_PASSANT_NO_SQUARE),
        Some((file, rank)) => {
            let playermask = if *rank == Rank::Six { 1u8 << 6 } else { 0u8 };
            let square: u8 = (7 - *file as u8) + (*rank as u8 * 8);
            EnPassantTarget(playermask | square)
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn(Player),
    Rook(Player),
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    King(Player),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
    pub black: PlayerState,
    pub white: PlayerState,
    pub en_passant_target: EnPassantTarget,
    pub active_player: Player,
    pub half_counter: u8,
    pub full_counter: u32, //pretty sure it's impossible to have a chess game go longer than 4 billion moves, but we'll see
    pub move_stack: Vec<(Move, UnmakeMoveData)>,
}

macro_rules! match_bits {
    ($square: ident) => {
       None
    };
    ($square: ident, $compared: expr => $result: expr) => {
        if $square & $compared != 0 {Some($result)} else {match_bits!{$square}}
    };
    ($square: ident, $compared: expr => $result: expr,) => {
        if $square & $compared != 0 {Some($result)} else {match_bits!{$square}}
    };
    ($square: ident, $compared: expr => $result: expr, $($rest:tt)*) => {
        if $square & $compared != 0 {Some($result)} else {match_bits!{$square, $($rest)*}}
    }
}

impl BoardState {
    pub fn new(game: ParsedGameState) -> Self {
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
            move_stack: vec![],
        };
        debug_assert!(result.is_valid());
        result
    }

    pub fn is_valid(&self) -> bool {
        self.white.is_valid() & self.black.is_valid()
    }

    pub fn all_pieces(&self) -> u64 {
        self.black.all_pieces() | self.white.all_pieces()
    }

    pub fn is_in_check(&self) -> bool {
        todo!()
    }

    pub fn player(&self) -> &PlayerState {
        match self.active_player {
            Player::Black => &self.black,
            Player::White => &self.white
        }
    }

    pub fn opponent(&self) -> &PlayerState {
        match self.active_player {
            Player::Black => &self.white,
            Player::White => &self.black
        }
    }

    pub fn find_piece_on_square(&self, square: u8) -> Option<PieceType> {
        let square_board: u64 = 1 << square;
        match_bits! {
            square_board,
            self.white.pawns => PieceType::Pawn(Player::White),
            self.white.bishops => PieceType::Bishop(Player::White),
            self.white.knights => PieceType::Knight(Player::White),
            self.white.rooks => PieceType::Rook(Player::White),
            self.white.queens => PieceType::Queen(Player::White),
            self.white.king => PieceType::King(Player::White),
            self.black.pawns => PieceType::Pawn(Player::Black),
            self.black.bishops => PieceType::Bishop(Player::Black),
            self.black.knights => PieceType::Knight(Player::Black),
            self.black.rooks => PieceType::Rook(Player::Black),
            self.black.queens => PieceType::Queen(Player::Black),
            self.black.king => PieceType::King(Player::Black),
        }
    }

    pub fn make_move(&mut self, to_move: Move) {
        let mut move_data = UnmakeMoveData {
            castling_rights: crate::gamestate::CastlingRights {
                black_kingside: self.black.king_castle,
                black_queenside: self.black.queen_castle,
                white_kingside: self.white.king_castle,
                white_queenside: self.white.queen_castle,
            },
            capture: CapturedPiece::None,
            en_passant: self.en_passant_target.clone(),
            halfmove_clock: self.half_counter,
        };
        let moved_piece = self
            .find_piece_on_square(to_move.from)
            .expect("cannot move from an empty square!");
        let captured_piece = self.find_piece_on_square(to_move.to);
        let (player, opponent) = match self.active_player {
            Player::Black => (&mut self.black, &mut self.white),
            Player::White => (&mut self.white, &mut self.black),
        };
        if (1 << to_move.to) & self.en_passant_target.targeted_square() != 0 {
            if let Some(player) = self.en_passant_target.targeted_player() {
                if let PieceType::Pawn(_) = moved_piece {
                    if self.active_player != player {
                        let pawn_square = if player == Player::Black {
                            to_move.to - 8
                        } else {
                            to_move.to + 8
                        };
                        move_data.capture = CapturedPiece::Pawn(pawn_square);
                    }
                }
            }
        } else if let Some(captured_piece) = captured_piece {
            match captured_piece {
                PieceType::Pawn(_) => {
                    move_data.capture = CapturedPiece::Pawn(to_move.to);
                }
                PieceType::Rook(_) => {
                    move_data.capture = CapturedPiece::Rook(to_move.to);
                    if opponent.king_castle && opponent.king.ilog2() as u8 > to_move.to 
                    && (opponent.king.ilog2() as u8)/8 == to_move.to/8 {
                        opponent.king_castle = false;
                    }
                    if opponent.queen_castle && to_move.to > opponent.king.ilog2() as u8
                    && (opponent.king.ilog2() as u8)/8 == to_move.to/8 {
                        opponent.queen_castle = false;
                    }
                }
                PieceType::Bishop(_) => {
                    move_data.capture = CapturedPiece::Bishop(to_move.to);
                }
                PieceType::Knight(_) => move_data.capture = CapturedPiece::Knight(to_move.to),
                PieceType::Queen(_) => move_data.capture = CapturedPiece::Queen(to_move.to),
                PieceType::King(_) => {
                    panic!()
                }
            }
        }
        match move_data.capture {
            CapturedPiece::None => self.half_counter += 1,
            CapturedPiece::Pawn(square) => {
                opponent.pawns &= !(1 << square);
                self.half_counter = 0;
            }
            CapturedPiece::Rook(square) => {
                opponent.rooks &= !(1 << square);
                self.half_counter = 0;
            }
            CapturedPiece::Bishop(square) => {
                opponent.bishops &= !(1 << square);
                self.half_counter = 0;
            }
            CapturedPiece::Knight(square) => {
                opponent.knights &= !(1 << square);
                self.half_counter = 0;
            }
            CapturedPiece::Queen(square) => {
                opponent.queens &= !(1 << square);
                self.half_counter = 0;
            }
        }
        match moved_piece {
            PieceType::Pawn(_) => {
                //0: no promotion, 1: knight, 2: bishop, 3: rook, 4: queen
                player.pawns &= !(1 << to_move.from);
                match to_move.promotion {
                    Promotion::None => player.pawns |= 1 << to_move.to,
                    Promotion::Knight => player.knights |= 1 << to_move.to,
                    Promotion::Bishop => player.bishops |= 1 << to_move.to,
                    Promotion::Rook => player.rooks |= 1 << to_move.to,
                    Promotion::Queen => player.queens |= 1 << to_move.to,
                }
                self.half_counter = 0;
            }
            PieceType::Rook(_) => {
                player.rooks &= !(1 << to_move.from);
                player.rooks |= 1 << to_move.to;
                if to_move.from as i16 - player.king.ilog2() as i16 > 0 {
                    player.queen_castle = false;
                } else {
                    player.king_castle = false;
                }
            }
            PieceType::Knight(_) => {
                player.knights &= !(1 << to_move.from);
                player.knights |= 1 << to_move.to;
            }
            PieceType::Bishop(_) => {
                player.bishops &= !(1 << to_move.from);
                player.bishops |= 1 << to_move.to;
            }
            PieceType::Queen(_) => {
                player.queens &= !(1 << to_move.from);
                player.queens |= 1 << to_move.to;
            }
            PieceType::King(active_player) => {
                if player.king_castle && 1 << to_move.to & 0x0200000000000002u64 != 0 {
                    player.rooks &= !(if active_player == Player::Black {0x0100000000000000} else {0x0000000000000001});
                    player.rooks |= 1 << (to_move.to + 1);
                }
                if player.queen_castle && 1 << to_move.to & 0x2000000000000020u64 != 0 {
                    player.rooks &= !(if active_player == Player::Black {0x8000000000000000} else {0x0000000000000080});
                    player.rooks |= 1 << (to_move.to - 1);
                }
                player.king &= !(1 << to_move.from);
                player.king |= 1 << to_move.to;

                player.queen_castle = false;
                player.king_castle = false;
                debug_assert_eq!(player.king.count_ones(), 1);
            }
        }

        match moved_piece {
            PieceType::Pawn(_) => {
                if to_move.from.abs_diff(to_move.to) == 16 {
                    self.en_passant_target = EnPassantTarget(
                        0x40 * (self.active_player == Player::Black) as u8
                            + ((to_move.from + to_move.to) / 2),
                    );
                } else {
                    self.en_passant_target = EnPassantTarget(EN_PASSANT_NO_SQUARE);
                }
            }
            _ => self.en_passant_target = EnPassantTarget(EN_PASSANT_NO_SQUARE),
        };
        self.active_player = if self.active_player == Player::Black {
            Player::White
        } else {
            Player::Black
        };
        self.full_counter += if self.active_player == Player::Black {
            1
        } else {
            0
        };
        self.move_stack.push((to_move, move_data))
    }

    pub fn unmake_last_move(&mut self) {
        if self.move_stack.last().is_none() {
            return;
        }

        let (last_move, move_data) = self.move_stack.pop().unwrap();
        self.active_player = match self.active_player {
            Player::Black => {
                self.full_counter -= 1;
                Player::White
            }
            Player::White => Player::Black,
        };

        let moved_piece = self
            .find_piece_on_square(last_move.to)
            .expect("cannot unmove from an empty square!");

        let (player, opponent) = if self.active_player == Player::Black {
            (&mut self.black, &mut self.white)
        } else {
            (&mut self.white, &mut self.black)
        };

        match moved_piece {
            PieceType::Pawn(_) => {
                //0: no promotion, 1: knight, 2: bishop, 3: rook, 4: queen
                player.pawns &= !(1 << last_move.to);
                player.pawns |= 1 << last_move.from;
            }
            PieceType::Rook(_) => {
                player.rooks &= !(1 << last_move.to);
                match last_move.promotion {
                    Promotion::None => player.rooks |= 1 << last_move.from,
                    Promotion::Rook => player.pawns |= 1 << last_move.from,
                    x => {
                        println!("{:?}", x);
                        unreachable!()
                    }
                }
            }
            PieceType::Bishop(_) => {
                player.bishops &= !(1 << last_move.to);
                match last_move.promotion {
                    Promotion::None => player.bishops |= 1 << last_move.from,
                    Promotion::Bishop => player.pawns |= 1 << last_move.from,
                    x => {
                        println!("{:?}", x);
                        unreachable!()
                    }
                }
            }
            PieceType::Knight(_) => {
                player.knights &= !(1 << last_move.to);
                match last_move.promotion {
                    Promotion::None => player.knights |= 1 << last_move.from,
                    Promotion::Knight => player.pawns |= 1 << last_move.from,
                    x => {
                        println!("{:?}", x);
                        unreachable!()
                    }
                }
            }
            PieceType::Queen(_) => {
                //0: no promotion, 1: knight, 2: bishop, 3: rook, 4: queen
                player.queens &= !(1 << last_move.to);
                match last_move.promotion {
                    Promotion::None => player.queens |= 1 << last_move.from,
                    Promotion::Queen => player.pawns |= 1 << last_move.from,
                    x => {
                        println!("{:?}", x);
                        unreachable!()
                    }
                }
            }
            PieceType::King(_) => {
                player.king &= !(1u64 << last_move.to);
                player.king |= 1u64 << last_move.from;
                if last_move.to as i16 - last_move.from as i16 == 2 {
                    player.rooks &= !(1u64 << (last_move.to - 1));
                    player.rooks |= 1u64 << ((last_move.to - File::from_square(last_move.to).to_u8()) + 7u8);
                } else if last_move.to as i16 - last_move.from as i16 == -2 {
                    player.rooks &= !(1u64 << (last_move.to + 1));
                    player.rooks |= 1u64 << (last_move.to - File::from_square(last_move.to).to_u8());
                }
                debug_assert_eq!(player.king.count_ones(), 1);
            }
        }

        match move_data.capture {
            CapturedPiece::None => (),
            CapturedPiece::Pawn(square) => opponent.pawns |= 1 << square,
            CapturedPiece::Knight(square) => opponent.knights |= 1 << square,
            CapturedPiece::Bishop(square) => opponent.bishops |= 1 << square,
            CapturedPiece::Rook(square) => opponent.rooks |= 1 << square,
            CapturedPiece::Queen(square) => opponent.queens |= 1 << square,
        }

        self.en_passant_target = move_data.en_passant;
        self.black.king_castle = move_data.castling_rights.black_kingside;
        self.black.queen_castle = move_data.castling_rights.black_queenside;
        self.white.king_castle = move_data.castling_rights.white_kingside;
        self.white.queen_castle = move_data.castling_rights.white_queenside;
        self.half_counter = move_data.halfmove_clock;
    }
    ///move_list is treated as a stack, and a slice of it containing the number of valid moves
    /// is returned by this function. the rest of move_list should be assumed to be invalid.
    pub fn generate_moves<'a>(&self, move_list: &'a mut [Move; 218]) -> &'a mut [Move] {
        let mut move_index = 0usize;

        let (player, opponent) = if self.active_player == Player::Black {
            (&self.black, &self.white)
        } else {
            (&self.white, &self.black)
        };

        let all_player_pieces = player.all_pieces();
        let all_opponent_pieces = opponent.all_pieces();
        let all_pieces = self.all_pieces();
        let king_square = player.king.ilog2() as u8;
        let attacked_squares: u64 = find_opponent_attacked_squares(
            all_pieces & !(player.king),
            opponent,
            &self.en_passant_target,
            self.active_player,
        );
        let mut king_move = king_moves(king_square, all_player_pieces);
        king_move &= !attacked_squares;
        insert_moves(king_square as u8, king_move, move_list, &mut move_index);
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
            0 => {
                //can only castle if the king is not in check.
                let castles = king_castles(
                    king_square,
                    all_pieces,
                    player.king_castle,
                    player.queen_castle,
                    attacked_squares,
                );
                insert_moves(king_square as u8, castles, move_list, &mut move_index);
            }
            1 => {
                capture_mask = checked_by;
                push_mask = 0;
                if (opponent.bishops | opponent.rooks | opponent.queens) & checked_by != 0 {
                    let square_checked_by = checked_by.ilog2() as u8;
                    if bishop_moves(square_checked_by , all_pieces, 0) & player.king != 0 {
                        push_mask |= bishop_moves(king_square, all_pieces, all_pieces)
                            & bishop_moves(checked_by.ilog2() as u8, all_pieces, all_pieces)
                    }
                    if rook_moves(square_checked_by , all_pieces, 0) & player.king != 0 {
                        push_mask |= rook_moves(king_square, all_pieces, all_pieces)
                            & rook_moves(checked_by.ilog2() as u8, all_pieces, all_pieces)
                    }
                }
            }
            _ => {
                //after 50 turns without a capture or pawn move, *and* there is at least one legal move,
                //the game is over by the fifty-move rule.
                if self.half_counter >= 100 && move_index > 0 {
                    return move_list.as_mut_slice().split_at_mut(0).0;
                }

                //check self.move_stack. if there are more than 6 entries, and the last 6 are the same 2 moves repeated,
                //the game is over by threefold repetition.
                let mut x = self.move_stack.iter().rev().take(6);
                if x.len() == 6 {
                    let (move1, _) = x.next().unwrap();
                    let (move2, _) = x.next().unwrap();
                    let mut is_draw = true;
                    for (move_, data) in x {
                        if data.capture != CapturedPiece::None || (move_ != move1 && move_ != move2)
                        {
                            is_draw = false;
                        }
                    }
                    if is_draw {
                        return move_list.as_mut_slice().split_at_mut(0).0;
                    }
                }
                return move_list.as_mut_slice().split_at_mut(move_index).0;
            }
        }
        let pinned_pieces = king_pins_bishop(
            king_square,
            all_player_pieces,
            all_pieces,
            opponent.bishops | opponent.queens,
        ) | king_pins_rook(
            king_square,
            all_player_pieces,
            all_pieces,
            opponent.rooks | opponent.queens,
        );

        generate_sliding_moves(player, all_pieces, all_player_pieces, capture_mask, push_mask, pinned_pieces, king_square, move_list, &mut move_index);
        generate_knight_moves(player, all_player_pieces, capture_mask, push_mask, pinned_pieces, move_list, &mut move_index);
        generate_pawn_moves(player, opponent, king_square, all_pieces, all_opponent_pieces, &self.en_passant_target, self.active_player, capture_mask, push_mask, pinned_pieces, move_list, &mut move_index);
        //after 50 turns without a capture or pawn move, *and* there is at least one legal move,
        //the game is over by the fifty-move rule.
        if self.half_counter >= 100 && move_index > 0 {
            return move_list.as_mut_slice().split_at_mut(0).0;
        }

        //check self.move_stack. if there are more than 6 entries, and the last 6 are the same 2 moves repeated,
        //the game is over by threefold repetition.
        let mut x = self.move_stack.iter().rev().take(6);
        if x.len() == 6 {
            let (move1, _) = x.next().unwrap();
            let (move2, _) = x.next().unwrap();
            let mut is_draw = true;
            for (move_, data) in x {
                if data.capture != CapturedPiece::None || (move_ != move1 && move_ != move2) {
                    is_draw = false;
                }
            }
            if is_draw {
                return move_list.as_mut_slice().split_at_mut(0).0;
            }
        }
        move_list.as_mut_slice().split_at_mut(move_index).0
    }
}