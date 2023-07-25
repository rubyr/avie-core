use crate::gamestate::{File, ParsedGameState, Player, Rank};

static FILE_A: u64 = 0x8080808080808080u64;
static NOT_FILE_A: u64 = !FILE_A;
static NOT_FILE_H: u64 = !FILE_H;
static FILE_H: u64 = 0x0101010101010101u64;
static RANK_1: u64 = 0x00000000000000FFu64;
static RANK_2: u64 = 0x000000000000FF00u64;
static RANK_7: u64 = 0x00FF000000000000u64;

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
static EN_PASSANT_NO_SQUARE: u8 = 0b10000000;
static EN_PASSANT_SQUARE_MASK: u8 = 0b00111111;

impl EnPassantTarget {
    fn targeted_player(&self) -> Option<Player> {
        match self.0 >> 6 {
            0 => Some(Player::White),
            1 => Some(Player::Black),
            //all other values mean there is no en passant target
            _ => None,
        }
    }
    fn targeted_square(&self) -> u64 {
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
    castling_rights: crate::gamestate::CastlingRights,
    capture: CapturedPiece,
    en_passant: EnPassantTarget,
    halfmove_clock: u8,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
    black: PlayerState,
    white: PlayerState,
    en_passant_target: EnPassantTarget,
    active_player: Player,
    half_counter: u8,
    full_counter: u32, //pretty sure it's impossible to have a chess game go longer than 4 billion moves, but we'll see
    move_stack: Vec<(Move, UnmakeMoveData)>,
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Promotion {
    None,
    Knight,
    Bishop,
    Rook,
    Queen
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    from: u8,
    to: u8,
    promotion: Promotion, //0: no promotion, 1: knight, 2: bishop, 3: rook, 4: queen
}

impl Move {
    fn new(from: u8, to: u8, promotion: Promotion) -> Move {
        Move {
            from,
            to,
            promotion,
        }
    }
}

fn en_passant_target(target: &Option<(File, Rank)>) -> EnPassantTarget {
    match target {
        None => EnPassantTarget(EN_PASSANT_NO_SQUARE),
        Some((file, rank)) => {
            let playermask = if *rank == Rank::Six {
                1u8 << 6
            } else {
                0u8
            };
            let square: u8 = (7 - *file as u8) + (*rank as u8 * 8);
            EnPassantTarget(playermask | square)
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum PieceType {
    Pawn(Player),
    Rook(Player),
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    King(Player),
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
        debug_assert_eq!(result.is_valid(), true);
        result
    }

    fn is_valid(&self) -> bool {
        self.white.is_valid() & self.black.is_valid()
    }

    fn all_pieces(&self) -> u64 {
        self.black.all_pieces() | self.white.all_pieces()
    }

    fn find_piece_on_square(&self, square: u8) -> Option<PieceType> {
        let square_board: u64 = 1 << square;
        match_bits!{
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

    fn make_move(&mut self, to_move: Move) {
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
        let old_king = player.king;
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
        } else {
            if captured_piece.is_some() {
                match captured_piece.unwrap() {
                    PieceType::Pawn(_) => {
                        move_data.capture = CapturedPiece::Pawn(to_move.to);
                    }
                    PieceType::Rook(_) => {
                        move_data.capture = CapturedPiece::Rook(to_move.to);
                        if opponent.king_castle && opponent.king.ilog2() as u8 > to_move.to {
                            opponent.king_castle = false;
                        }
                        if opponent.queen_castle && to_move.to > opponent.king.ilog2() as u8 {
                            opponent.queen_castle = false;
                        }
                    }
                    PieceType::Bishop(_) => {
                        move_data.capture = CapturedPiece::Bishop(to_move.to);
                    }
                    PieceType::Knight(_) => {
                        move_data.capture = CapturedPiece::Knight(to_move.to)
                    }
                    PieceType::Queen(_) => {
                        move_data.capture = CapturedPiece::Queen(to_move.to)
                    }
                    PieceType::King(_) => {
                        panic!()
                    }
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
                if player.king_castle == true && 1 << to_move.to & 0x0200000000000002u64 != 0 {
                    player.rooks &= !(0x0100000000000001);
                    player.rooks |= 1 << (to_move.to + 1);
                }
                if player.queen_castle == true && 1 << to_move.to & 0x2000000000000020u64 != 0 {
                    player.rooks &= !(0x8000000000000080);
                    player.rooks |= 1 << (to_move.to - 1);
                }
                player.king &= !(1 << to_move.from);
                player.king |= 1 << to_move.to;

                player.queen_castle = false;
                player.king_castle = false;
                if player.king.count_ones() != 1 {
                    eprintln!("active player: {:?}", active_player);
                    eprintln!("actual active player: {:?}", self.active_player);
                    eprintln!("kings: {}", player.king.count_ones());
                    for i in 0..=7u8 {
                        let row = (player.king >> (56 - (i * 8))) as u8;
                        eprintln!("{:08b}", row);
                    }
                    eprintln!("");
                    for i in 0..=7u8 {
                        let row = (old_king >> (56 - (i * 8))) as u8;
                        eprintln!("{:08b}", row);
                    }
                    //println!("move string: {}", move_str);
                    eprintln!("move struct: {:?}", to_move);
                    eprintln!("{:?}", self.move_stack);
                    panic!();
                }
            }
        }

        match moved_piece {
            PieceType::Pawn(_) => {
                if to_move.from.abs_diff(to_move.to) == 16 {
                    self.en_passant_target = EnPassantTarget(
                        0x40 * (self.active_player == Player::Black) as u8
                            + ((to_move.from + to_move.to) / 2) as u8,
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

    fn unmake_last_move(&mut self) {
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
            .expect("cannot move from an empty square!");

        let (player, opponent) = if self.active_player == Player::Black {
            (&mut self.black, &mut self.white)
        } else {
            (&mut self.white, &mut self.black)
        };

        let old_king = player.king;

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
                        println!("{:?}",x);
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
                        println!("{:?}",x);
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
                        println!("{:?}",x);
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
                        println!("{:?}",x);
                        unreachable!()
                    }
                }
            }
            PieceType::King(active_player) => {
                player.king &= !(1 << last_move.to);
                player.king |= 1 << last_move.from;
                if last_move.to as i16 - last_move.from as i16 == 2 {
                    player.rooks &= !(1 << last_move.to - 1);
                    player.rooks |= 1 << (last_move.to - last_move.to % 8) + 7;
                } else if last_move.to as i16 - last_move.from as i16 == -2 {
                    player.rooks &= !(1 << last_move.to + 1);
                    player.rooks |= 1 << (last_move.to - last_move.to % 8);
                }

                if player.king.count_ones() != 1 {
                    eprintln!("active player: {:?}", active_player);
                    eprintln!("actual active player: {:?}", self.active_player);
                    eprintln!("kings: {}", player.king.count_ones());
                    for i in 0..=7u8 {
                        let row = (player.king >> (56 - (i * 8))) as u8;
                        println!("{:08b}", row);
                    }
                    eprintln!("");
                    for i in 0..=7u8 {
                        let row = (old_king >> (56 - (i * 8))) as u8;
                        println!("{:08b}", row);
                    }
                    //println!("move string: {:?}", move_str);
                    eprintln!("move struct: {:?}", last_move);
                    eprintln!("{:?}", self.move_stack);
                    panic!();
                }
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
            all_pieces & !(player.king),
            opponent,
            &self.en_passant_target,
            self.active_player,
        );
        let mut king_move = king_moves(king_square, player.all_pieces());
        king_move = (king_move
            | king_castles(
                king_square,
                self.all_pieces(),
                player.king_castle,
                player.queen_castle,
                attacked_squares,
            ))
            & !attacked_squares;
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
        let mut en_passant_mask = 0x0u64;

        match checked_by.count_ones() {
            0 => {}
            1 => {
                capture_mask = checked_by;
                push_mask = 0; 
                if checked_by & opponent.pawns != 0 {
                    if self.active_player == Player::Black {
                        if self.en_passant_target.targeted_square() & (opponent.pawns >> 8) != 0 {
                            en_passant_mask = self.en_passant_target.targeted_square();
                        }
                    } else {
                        if self.en_passant_target.targeted_square() & (opponent.pawns << 8) != 0 {
                            en_passant_mask = self.en_passant_target.targeted_square();
                        }
                    }
                };
                if (opponent.bishops | opponent.rooks | opponent.queens) & checked_by != 0 {
                    if bishop_moves(checked_by.ilog2() as usize, all_pieces, 0) & player.king != 0 {
                        push_mask |= bishop_moves(king_square, all_pieces, all_pieces)
                            & bishop_moves(checked_by.ilog2() as usize, all_pieces, all_pieces)
                    }
                    if rook_moves(checked_by.ilog2() as usize, all_pieces, 0) & player.king != 0 {
                        push_mask |= rook_moves(king_square, all_pieces, all_pieces)
                            & rook_moves(checked_by.ilog2() as usize, all_pieces, all_pieces)
                    }
                }
            }
            _ => {
                //after 50 turns without a capture or pawn move, *and* there is at least one legal move,
                //the game is over by the fifty-move rule.
                if self.half_counter >= 100 && move_index > 0{
                    return move_list.as_mut_slice().split_at_mut(0).0
                }
            
                //check self.move_stack. if there are more than 6 entries, and the last 6 are the same 2 moves repeated,
                //the game is over by threefold repetition.
                let x = self.move_stack.iter().rev().take(6).collect::<Box<[_]>>();
                if x.len() == 6 {
                    let (move1, _) = x[0];
                    let (move2, _) = x[1];
                    let mut is_draw = true;
                    for (move_, data) in x.iter() {
                        if data.capture != CapturedPiece::None || (move_ != move1 && move_ != move2) {
                            is_draw = false;
                        }
                    }
                    if is_draw {
                        return move_list.as_mut_slice().split_at_mut(0).0
                    }
                }
                return move_list.as_mut_slice().split_at_mut(move_index).0
            },
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
        for rook in BitboardIterator::new(player.rooks) {
            let mut moves = rook_moves(rook as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << rook & pinned_pieces != 0 {
                if bishop_moves(rook as usize, 0, 0) & player.king != 0 {
                    moves &= bishop_moves(king_square, 0, 0) & bishop_moves(rook as usize, 0, 0)
                }
                if rook_moves(rook as usize, 0, 0) & player.king != 0 {
                    moves &= rook_moves(king_square, 0, 0) & rook_moves(rook as usize, 0, 0)
                }
            }
            insert_moves(rook, moves, move_list, &mut move_index);
        }
        for bishop in BitboardIterator::new(player.bishops) {
            let mut moves = bishop_moves(bishop as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << bishop & pinned_pieces != 0 {
                if bishop_moves(bishop as usize, 0, 0) & player.king != 0 {
                    moves &= bishop_moves(king_square, 0, 0) & bishop_moves(bishop as usize, 0, 0)
                }
                if rook_moves(bishop as usize, 0, 0) & player.king != 0 {
                    moves &= rook_moves(king_square, 0, 0) & rook_moves(bishop as usize, 0, 0)
                }
            }
            insert_moves(bishop, moves, move_list, &mut move_index);
        }
        for knight in BitboardIterator::new(player.knights) {
            let moves =
                knight_moves(knight as usize, player.all_pieces()) & (capture_mask | push_mask);
            if 1 << knight & pinned_pieces == 0 {
                insert_moves(knight, moves, move_list, &mut move_index);
            }
        }
        for queen in BitboardIterator::new(player.queens) {
            let mut moves = queen_moves(queen as usize, all_pieces, player.all_pieces())
                & (capture_mask | push_mask);
            if 1 << queen & pinned_pieces != 0 {
                if bishop_moves(queen as usize, 0, 0) & player.king != 0 {
                    moves &= bishop_moves(king_square, 0, 0) & bishop_moves(queen as usize, 0, 0)
                }
                if rook_moves(queen as usize, 0, 0) & player.king != 0 {
                    moves &= rook_moves(king_square, 0, 0) & rook_moves(queen as usize, 0, 0)
                }
            }
            insert_moves(queen, moves, move_list, &mut move_index);
        }
        for pawn in BitboardIterator::new(player.pawns) {
            let mut attacks = pawn_attacks(
                1 << pawn,
                opponent.all_pieces(),
                self.en_passant_target.clone(),
                self.active_player,
            );
            if attacks[0] & self.en_passant_target.targeted_square() != 0 {
                if king_attacked_rooks(
                    king_square,
                    (all_pieces | self.en_passant_target.targeted_square())
                        & !(1u64 << pawn | 1u64 << (pawn + 1)),
                    opponent.rooks | opponent.queens,
                ) != 0
                {
                    attacks[0] &= !(self.en_passant_target.targeted_square());
                }
            }
            if attacks[1] & self.en_passant_target.targeted_square() != 0 {
                if king_attacked_rooks(
                    king_square,
                    (all_pieces | self.en_passant_target.targeted_square())
                        & !(1u64 << pawn | 1u64 << (pawn - 1)),
                    opponent.rooks | opponent.queens,
                ) != 0
                {
                    attacks[1] &= !(self.en_passant_target.targeted_square());
                }
            }
            let mut moves = (pawn_single_pushes(1u64 << pawn, all_pieces, self.active_player)
                | pawn_double_pushes(1 << pawn, all_pieces, self.active_player)
                | attacks[0]
                | attacks[1])
                & (capture_mask | push_mask | en_passant_mask);
            if 1 << pawn & pinned_pieces != 0 {
                if bishop_moves(pawn as usize, 0, 0) & player.king != 0 {
                    moves &= bishop_moves(king_square, 0, 0) & bishop_moves(pawn as usize, 0, 0)
                }
                if rook_moves(pawn as usize, 0, 0) & player.king != 0 {
                    moves &= rook_moves(king_square, 0, 0) & rook_moves(pawn as usize, 0, 0)
                }
            }
            let move_squares = BitboardIterator::new(moves);
            for move_ in move_squares {
                if (1u64 << move_)
                    & if self.active_player == Player::Black {
                        0xFF
                    } else {
                        0xFF00000000000000
                    }
                    != 0
                {
                    move_list[move_index] = Move::new(
                        pawn,
                        move_,
                        Promotion::Knight,
                    );
                    move_index += 1;
                    move_list[move_index] = Move::new(
                        pawn,
                        move_,
                        Promotion::Bishop,
                    );
                    move_index += 1;
                    move_list[move_index] = Move::new(
                        pawn,
                        move_,
                        Promotion::Rook,
                    );
                    move_index += 1;
                    move_list[move_index] = Move::new(
                        pawn,
                        move_,
                        Promotion::Queen,
                    );
                    move_index += 1;
                } else {
                    move_list[move_index] = Move::new(
                        pawn,
                        move_,
                        Promotion::None,
                    );
                    move_index += 1;
                }
            }
        }
        //after 50 turns without a capture or pawn move, *and* there is at least one legal move,
        //the game is over by the fifty-move rule.
        if self.half_counter >= 100 && move_index > 0{
            return move_list.as_mut_slice().split_at_mut(0).0
        }

        //check self.move_stack. if there are more than 6 entries, and the last 6 are the same 2 moves repeated,
        //the game is over by threefold repetition.
        let x = self.move_stack.iter().rev().take(6).collect::<Box<[_]>>();
        if x.len() == 6 {
            let (move1, _) = x[0];
            let (move2, _) = x[1];
            let mut is_draw = true;
            for (move_, data) in x.iter() {
                if data.capture != CapturedPiece::None || (move_ != move1 && move_ != move2) {
                    is_draw = false;
                }
            }
            if is_draw {
                return move_list.as_mut_slice().split_at_mut(0).0
            }
        }
        move_list.as_mut_slice().split_at_mut(move_index).0
    }
}

struct BitboardIterator(u64);

impl BitboardIterator {
    #[inline]
    fn new(board: u64) -> Self {
        Self(board)
    }
}

impl Iterator for BitboardIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.count_ones() == 0 {return None}
        let result = self.0.trailing_zeros() as u8;
        self.0 &= !(1 << result);
        Some(result)
    }
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
        if active_player == Player::Black {
            Player::White
        } else {
            Player::Black
        },
    );
    let mut attacked_squares = 0;
    attacked_squares |= BitboardIterator::new(opponent.rooks)
        .fold(0, |x, y| x | rook_moves(y as usize, all_pieces, 0));
    attacked_squares |= BitboardIterator::new(opponent.bishops)
        .fold(0, |x, y| x | bishop_moves(y as usize, all_pieces, 0));
    attacked_squares |= BitboardIterator::new(opponent.queens)
        .fold(0, |x, y| x | queen_moves(y as usize, all_pieces, 0));
    attacked_squares |= BitboardIterator::new(opponent.knights)
        .fold(0, |x, y| x | knight_moves(y as usize, 0));
    attacked_squares |= king_moves(opponent.king.ilog2() as usize, 0);
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
        if active_player == Player::Black {
            Player::White
        } else {
            Player::Black
        },
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
    let move_squares = BitboardIterator::new(moves);
    for move_ in move_squares {
        move_list[*move_index] = Move {
            from: piece,
            to: move_,
            promotion: Promotion::None,
        };
        *move_index += 1;
    }
}

///Pawns are able to push forward when there is no piece blocking their way
fn pawn_single_pushes(pawns: u64, all_pieces: u64, player: Player) -> u64 {
    if player == Player::Black {
        (pawns >> 8) & (!all_pieces)
    } else {
        (pawns << 8) & (!all_pieces)
    }
}
fn pawn_double_pushes(pawns: u64, all_pieces: u64, player: Player) -> u64 {
    if player == Player::Black {
        if pawns & RANK_7 != 0 {
            ((pawns & RANK_7) >> 16) & (!all_pieces) & (!(all_pieces >> 8))
        } else {
            0
        }
    } else {
        if pawns & RANK_2 != 0 {
            ((pawns & RANK_2) << 16) & (!all_pieces) & (!(all_pieces << 8))
        } else {
            0
        }
    }
}
///attacks are stored in an array [west_attacks, east_attacks] from white's perspective
fn pawn_attacks(
    pawns: u64,
    enemy_pieces: u64,
    en_passant_target: EnPassantTarget,
    player: Player,
) -> [u64; 2] {
    let target = if Some(player) != en_passant_target.targeted_player() {
        en_passant_target.targeted_square()
    } else {
        0
    };

    //println!("{}", en_passant_target.targeted_square());
    if player == Player::Black {
        let west_attack_squares = (pawns >> 7) & NOT_FILE_H;
        let east_attack_squares = (pawns >> 9) & NOT_FILE_A;
        [
            west_attack_squares & (enemy_pieces | target),
            east_attack_squares & (enemy_pieces | target),
        ]
    } else {
        let west_attack_squares = (pawns << 9) & NOT_FILE_H;
        let east_attack_squares = (pawns << 7) & NOT_FILE_A;
        [
            west_attack_squares & (enemy_pieces | target),
            east_attack_squares & (enemy_pieces | target),
        ]
    }
}

fn king_moves(square: usize, friendly_pieces: u64) -> u64 {
    crate::precomputed::precomputed::KING_MOVES[square] & !friendly_pieces
}

fn king_castles(
    square: usize,
    all_pieces: u64,
    king_castle: bool,
    queen_castle: bool,
    attacked_squares: u64,
) -> u64 {
    let mut result = 0;
    if king_castle
        && (1u64 << square | 1u64 << square - 1 | 1u64 << square - 2) & attacked_squares == 0
        && (rook_moves(square, all_pieces, all_pieces)
            & rook_moves(square - 3, all_pieces, all_pieces))
            .count_ones()
            == 2
    {
        result |= 1 << ((square - square % 8) + 1);
    };
    if queen_castle
        && (1u64 << square | 1u64 << square + 1 | 1u64 << square + 2) & attacked_squares == 0
        && (rook_moves(square, all_pieces, all_pieces)
            & rook_moves(square + 4, all_pieces, all_pieces))
            .count_ones()
            == 3
    {
        result |= 1 << ((square - square % 8) + 5);
    };
    result
}

fn king_attacked_rooks(king: usize, all_pieces: u64, rooks_and_queens: u64) -> u64 {
    let mut new_rooks_and_queens = rooks_and_queens;
    let mut index = rooks_and_queens.trailing_zeros();
    let king_moves = rook_moves(king, rooks_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_rooks_and_queens != 0 {
        new_rooks_and_queens = (rooks_and_queens >> index) & !1;
        let moves = rook_moves(index as usize, 1 << king, 0);
        if (moves & (king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
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
        if (moves & &(king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
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
        if (moves & &(king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
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
        if (moves & &(king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
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
    crate::precomputed::precomputed::KNIGHT_MOVES[square] & !friendly_pieces
}

fn bishop_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let magic = crate::precomputed::bishop_magic::BISHOP_MAGICS[square];
    let blockers = all_pieces & crate::precomputed::precomputed::BISHOP_MASK[square];
    let magic_index = crate::precomputed::magic_to_index(magic, blockers, 9);
    crate::precomputed::bishop_magic::BISHOP_ATTACKS[square][magic_index] & !friendly_pieces
}

fn rook_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let magic = crate::precomputed::rook_magic::ROOK_MAGICS[square];
    let blockers = all_pieces & crate::precomputed::precomputed::ROOK_MASK[square];
    let magic_index = crate::precomputed::magic_to_index(magic, blockers, 12);
    crate::precomputed::rook_magic::ROOK_ATTACKS[square][magic_index] & !friendly_pieces
}

fn queen_moves(square: usize, all_pieces: u64, friendly_pieces: u64) -> u64 {
    bishop_moves(square, all_pieces, friendly_pieces) | rook_moves(square, all_pieces, friendly_pieces)
}

fn move_to_algebraic(move_: &Move, board: &BoardState) -> String {
    static SQUARES: [&'static str; 64] = [
        "h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1", "h2", "g2", "f2", "e2", "d2", "c2", "b2",
        "a2", "h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3", "h4", "g4", "f4", "e4", "d4", "c4",
        "b4", "a4", "h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5", "h6", "g6", "f6", "e6", "d6",
        "c6", "b6", "a6", "h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7", "h8", "g8", "f8", "e8",
        "d8", "c8", "b8", "a8",
    ];
    static FILES: [&'static str; 8] = ["h", "g", "f", "e", "d", "c", "b", "a"];
    static RANKS: [&'static str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
    let file_mask = FILE_H << move_.from % 8;
    let rank_mask = RANK_1 << move_.from / 8;
    let mut rank = "";
    let mut file = "";
    let mut promotion = "";
    let to_square = SQUARES[move_.to as usize];
    let piece_square = 1u64 << move_.from;
    let piece = if piece_square & board.black.rooks != 0 {
        if (board.black.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.bishops & bishop_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "r"
    } else if piece_square & board.black.bishops != 0 {
        if (board.black.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.bishops & bishop_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "b"
    } else if piece_square & board.black.knights != 0 {
        if (board.black.knights & knight_moves(move_.to as usize, 0) & rank_mask).count_ones() > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.knights & knight_moves(move_.to as usize, 0) & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.knights & knight_moves(move_.to as usize, 0)).count_ones() > 1 {
            file = FILES[move_.from as usize % 8];
        }
        "n"
    } else if piece_square & board.black.queens != 0 {
        if (board.black.queens & queen_moves(move_.to as usize, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.queens
            & queen_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.black.queens & queen_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "q"
    } else if piece_square & board.black.king != 0 {
        "k"
    } else if piece_square & board.black.pawns != 0 {
        let attacks = pawn_attacks(
            1u64 << move_.to,
            !0,
            board.en_passant_target.clone(),
            Player::White,
        );
        if (1u64 << move_.from & (attacks[0] | attacks[1])) != 0
            && (board.black.pawns & (attacks[0] | attacks[1])) != 0
        {
            file = FILES[move_.from as usize % 8];
        }
        promotion = match move_.promotion {
            Promotion::Knight => "=n",
            Promotion::Bishop => "=b",
            Promotion::Rook => "=r",
            Promotion::Queen => "=q",
            _ => "",
        };
        ""
    } else if piece_square & board.white.rooks != 0 {
        if (board.white.rooks & rook_moves(move_.to as usize, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.rooks
            & rook_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.rooks & rook_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "R"
    } else if piece_square & board.white.bishops != 0 {
        if (board.white.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.bishops
            & bishop_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.bishops & bishop_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "B"
    } else if piece_square & board.white.knights != 0 {
        if (board.white.knights & knight_moves(move_.to as usize, 0) & rank_mask).count_ones() > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.knights & knight_moves(move_.to as usize, 0) & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.knights & knight_moves(move_.to as usize, 0)).count_ones() > 1 {
            file = FILES[move_.from as usize % 8];
        }
        "N"
    } else if piece_square & board.white.queens != 0 {
        if (board.white.queens & queen_moves(move_.to as usize, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to as usize, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[move_.from as usize % 8];
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.queens
            & queen_moves(move_.to as usize, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[move_.from as usize / 8];
        } else if (board.white.queens & queen_moves(move_.to as usize, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[move_.from as usize % 8];
        }
        "Q"
    } else if piece_square & board.white.king != 0 {
        "K"
    } else if piece_square & board.white.pawns != 0 {
        let attacks = pawn_attacks(
            1u64 << move_.to,
            !0,
            board.en_passant_target.clone(),
            Player::Black,
        );
        if (1u64 << move_.from & (attacks[0] | attacks[1])) != 0
            && (board.white.pawns & (attacks[0] | attacks[1])) != 0
        {
            file = FILES[move_.from as usize % 8];
        }
        promotion = match move_.promotion {
            Promotion::Knight => "=N",
            Promotion::Bishop => "=B",
            Promotion::Rook => "=R",
            Promotion::Queen => "=Q",
            _ => "",
        };
        ""
    } else {
        "ERROR"
    };

    let capture = if 1u64 << move_.to & board.all_pieces() != 0 {
        "x"
    } else {
        ""
    };
    piece.to_owned() + file + rank + capture + to_square + promotion
}

pub fn perft(board: &mut BoardState, depth: u8) -> u64 {
    let mut move_count = 0;
    let mut move_array = [Move {
        from: 0,
        to: 0,
        promotion: Promotion::None,
    }; 218];
    let moves = board.generate_moves(&mut move_array);
    if depth == 1 {
        return moves.len() as u64;
    };
    for player_move in moves {
        board.make_move(*player_move);
        move_count += perft(board, depth - 1);
        board.unmake_last_move();
    }
    move_count
}

pub fn perft_div(board: &mut BoardState, depth: u8) -> u64 {
    let mut move_count = 0;
    let mut move_array = [Move {
        from: 0,
        to: 0,
        promotion: Promotion::None,
    }; 218];
    let moves = board.generate_moves(&mut move_array);
    if depth == 1 {
        for player_move in moves.iter().rev() {
            let move_string = move_to_algebraic(player_move, board);
            println!("{}: 1", move_string);
        }
        return moves.len() as u64;
    };
    let move_strings: Vec<_> = moves.iter().rev().map(|x| move_to_algebraic(x, board)).collect();
    for (i, player_move) in moves.iter().rev().enumerate() {
        
        board.make_move(*player_move);
        let result = perft(board, depth - 1);
        println!("{}: {}", move_strings[i], result);
        move_count += result;
        board.unmake_last_move();
    }
    move_count
}
