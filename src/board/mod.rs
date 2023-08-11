use crate::gamestate::{File, Player, Rank};
mod tests;
mod representation;

pub use crate::board::representation::*;

static FILE_A: u64 = 0x8080808080808080u64;
static NOT_FILE_A: u64 = !FILE_A;
static NOT_FILE_H: u64 = !FILE_H;
static FILE_H: u64 = 0x0101010101010101u64;
static RANK_1: u64 = 0x00000000000000FFu64;
static RANK_2: u64 = 0x000000000000FF00u64;
static RANK_7: u64 = 0x00FF000000000000u64;

pub fn u64_to_board(board: u64) {
    for i in 0..8 {
        let row = (board >> (56 - (i * 8))) as u8;
        println!("{:08b}", row);
    }
    println!()
}

pub struct BitboardIterator(u64);

impl BitboardIterator {
    pub fn new(board: u64) -> Self {
        Self(board)
    }
}

impl Iterator for BitboardIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = self.0.trailing_zeros() as u8;
            self.0 &= !(1 << result);
            Some(result)
        }
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
        .fold(0, |x, y| x | rook_moves(y , all_pieces, 0));
    attacked_squares |= BitboardIterator::new(opponent.bishops)
        .fold(0, |x, y| x | bishop_moves(y , all_pieces, 0));
    attacked_squares |= BitboardIterator::new(opponent.queens)
        .fold(0, |x, y| x | queen_moves(y , all_pieces, 0));
    attacked_squares |=
        BitboardIterator::new(opponent.knights).fold(0, |x, y| x | knight_moves(y , 0));
    attacked_squares |= king_moves(opponent.king.ilog2() as u8, 0);
    attacked_squares |= pawn_attacked_squares[0] | pawn_attacked_squares[1];
    attacked_squares
}

fn generate_sliding_moves(player: &PlayerState, all_pieces: u64, all_player_pieces: u64, capture_mask: u64, push_mask: u64, pinned_pieces: u64, loud_mask: u64, king_square: u8, move_list: &mut [Move;218], move_index: &mut usize) {
    for rook in BitboardIterator::new(player.rooks) {
        let mut moves = rook_moves(rook , all_pieces, all_player_pieces)
            & (capture_mask | push_mask);
        if 1 << rook & pinned_pieces != 0 {
            if bishop_moves(rook , 0, 0) & player.king != 0 {
                moves &= bishop_moves(king_square, 0, 0) & bishop_moves(rook , 0, 0)
            }
            if rook_moves(rook , 0, 0) & player.king != 0 {
                moves &= rook_moves(king_square, 0, 0) & rook_moves(rook , 0, 0)
            }
        }
        insert_moves(rook, moves & loud_mask, move_list, move_index);
    }
    for bishop in BitboardIterator::new(player.bishops) {
        let mut moves = bishop_moves(bishop , all_pieces, all_player_pieces)
            & (capture_mask | push_mask);
        if 1 << bishop & pinned_pieces != 0 {
            if bishop_moves(bishop , 0, 0) & player.king != 0 {
                moves &= bishop_moves(king_square, 0, 0) & bishop_moves(bishop , 0, 0)
            }
            if rook_moves(bishop , 0, 0) & player.king != 0 {
                moves &= rook_moves(king_square, 0, 0) & rook_moves(bishop , 0, 0)
            }
        }
        insert_moves(bishop, moves & loud_mask, move_list, move_index);
    }
    for queen in BitboardIterator::new(player.queens) {
        let mut moves = queen_moves(queen , all_pieces, all_player_pieces)
            & (capture_mask | push_mask);
        if 1 << queen & pinned_pieces != 0 {
            if bishop_moves(queen , 0, 0) & player.king != 0 {
                moves &= bishop_moves(king_square, 0, 0) & bishop_moves(queen , 0, 0)
            }
            if rook_moves(queen , 0, 0) & player.king != 0 {
                moves &= rook_moves(king_square, 0, 0) & rook_moves(queen , 0, 0)
            }
        }
        insert_moves(queen, moves & loud_mask, move_list, move_index);
    }
}

fn generate_knight_moves(player: &PlayerState, all_player_pieces: u64, capture_mask: u64, push_mask: u64, pinned_pieces: u64, loud_mask: u64, move_list: &mut [Move; 218], move_index:&mut usize) {
    for knight in BitboardIterator::new(player.knights) {
        let moves =
            knight_moves(knight , all_player_pieces) & (capture_mask | push_mask);
        if 1 << knight & pinned_pieces == 0 {
            insert_moves(knight, moves & loud_mask, move_list, move_index);
        }
    }
}

fn pawn_promotion_moves(pawn: u8, move_: u8, move_list: &mut [Move; 218], move_index: &mut usize, active_player: Player) {
    if (1u64 << move_)
            & if active_player == Player::Black {
                0xFF
            } else {
                0xFF00000000000000
            }
            != 0
        {
            move_list[*move_index] = Move::new(pawn, move_, Promotion::Knight);
            *move_index += 1;
            move_list[*move_index] = Move::new(pawn, move_, Promotion::Bishop);
            *move_index += 1;
            move_list[*move_index] = Move::new(pawn, move_, Promotion::Rook);
            *move_index += 1;
            move_list[*move_index] = Move::new(pawn, move_, Promotion::Queen);
            *move_index += 1;
        } else {
            move_list[*move_index] = Move::new(pawn, move_, Promotion::None);
            *move_index += 1;
        }
}
//todo!: ensure that all moves that could go to edge of board generate proper promotions!
fn generate_pawn_moves(player: &PlayerState, opponent: &PlayerState, king_square: u8, all_pieces: u64, all_opponent_pieces: u64, en_passant_target: &EnPassantTarget, active_player: Player,capture_mask: u64, push_mask: u64, pinned_pieces: u64, loud_mask: u64, move_list: &mut [Move; 218], move_index: &mut usize) {
    let mut pawns = player.pawns;
    let pinned_pawns = pawns & pinned_pieces;
    let offset: i8 = match active_player {
        Player::Black => 8,
        Player::White => -8
    };
    if pinned_pawns != 0 {
        pawns &= !pinned_pawns;
        let bishop_pin_mask = bishop_moves(king_square, 0, 0);
        if pinned_pawns & bishop_pin_mask != 0 {
            let mut attacks = pawn_attacks(pinned_pawns, all_opponent_pieces, en_passant_target.clone(), active_player);
            attacks[0] &= bishop_pin_mask & (capture_mask | push_mask);
            attacks[1] &= bishop_pin_mask & (capture_mask | push_mask);
            if attacks[0] != 0 {
                for move_ in BitboardIterator::new(attacks[0] & loud_mask) {
                    let pawn = (move_ as i8 + (offset - 1)) as u8;
                    pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
                }
            }
            if attacks[1] != 0 {
                for move_ in BitboardIterator::new(attacks[1] & loud_mask) {
                    let pawn = (move_ as i8 + (offset + 1)) as u8;
                        pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
                }
            }
        }
        let rook_pin_mask = rook_moves(king_square, 0, 0);
        if pinned_pawns & rook_pin_mask != 0 {
            let push = pawn_single_pushes(pinned_pawns, all_pieces, active_player) & rook_pin_mask & (capture_mask | push_mask);
            if push != 0 {
                for move_ in BitboardIterator::new(push & loud_mask) {
                    let pawn = (move_ as i8 + offset) as u8;
                    pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
                }
            }
            let double_push = pawn_double_pushes(pinned_pawns, all_pieces, active_player) & rook_pin_mask & (capture_mask | push_mask);
            if double_push != 0 {
                for move_ in BitboardIterator::new(double_push & loud_mask) {
                    let pawn = (move_ as i8 + offset*2) as u8;
                    pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
                }
            }
        }
    }
    let mut attacks = pawn_attacks(pawns, all_opponent_pieces, en_passant_target.clone(), active_player);
    
    if attacks[0] & en_passant_target.targeted_square() != 0 {
        let removed_pieces = (1u64 << (en_passant_target.targeted_square().ilog2() as i8 + offset)) + (1u64 << (en_passant_target.targeted_square().ilog2() as i8 + offset - 1));
        //todo!() some kind of issue here? not sure how the king can see rooks/queens here?
        if rook_moves(king_square, all_pieces & !removed_pieces, player.all_pieces() & !removed_pieces) & (opponent.rooks | opponent.queens) != 0 {
            attacks[0] &= !en_passant_target.targeted_square();
        }
    }
    if attacks[1] & en_passant_target.targeted_square() != 0 {
        let removed_pieces = (1u64 << (en_passant_target.targeted_square().ilog2() as i8 + offset)) + 1u64 << (en_passant_target.targeted_square().ilog2() as i8 + offset + 1);
        if rook_moves(king_square, all_pieces & !removed_pieces, player.all_pieces() & !removed_pieces) & (opponent.rooks | opponent.queens) != 0 {
            attacks[1] &= !en_passant_target.targeted_square();
        }
    }
    let single_pushes = pawn_single_pushes(pawns, all_pieces, active_player) & push_mask;
    let double_pushes = pawn_double_pushes(pawns, all_pieces, active_player) & push_mask;
    let west_attacks = BitboardIterator::new(attacks[0] & capture_mask & loud_mask);
    let east_attacks = BitboardIterator::new(attacks[1] & capture_mask & loud_mask);
    let pushes = BitboardIterator::new(single_pushes & loud_mask);
    let double_pushes = BitboardIterator::new(double_pushes & loud_mask);
    
    for move_ in west_attacks {
        let pawn = (move_ as i8 + (offset - 1)) as u8;
        pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
    }
    for move_ in east_attacks {
        let pawn = (move_ as i8 + (offset + 1)) as u8;
        pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
    }
    for move_ in pushes {
        let pawn = (move_ as i8 + offset) as u8;
        pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
    }
    for move_ in double_pushes {
        let pawn = (move_ as i8 + offset*2) as u8;
        pawn_promotion_moves(pawn, move_, move_list, move_index, active_player);
    }
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
        player.king.ilog2() as u8,
        all_pieces,
        opponent.rooks | opponent.queens,
    ) | king_attacked_bishops(
        player.king.ilog2() as u8,
        all_pieces,
        opponent.bishops | opponent.queens,
    ) | king_attacked_knights(player.king.ilog2() as u8, opponent.knights)
        | pawns_attacking_king
}

#[inline(never)]
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
    } else if pawns & RANK_2 != 0 {
        ((pawns & RANK_2) << 16) & (!all_pieces) & (!(all_pieces << 8))
    } else {
        0
    }
}
///attacks are stored in an array [west_attacks, east_attacks] from white's perspective
pub fn pawn_attacks(
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

fn king_moves(square: u8, friendly_pieces: u64) -> u64 {
    crate::precomputed::moves::KING_MOVES[square as usize] & !friendly_pieces
}

fn king_castles(
    square: u8,
    all_pieces: u64,
    king_castle: bool,
    queen_castle: bool,
    attacked_squares: u64,
) -> u64 {
    let mut result = 0;
    if king_castle
        && (1u64 << square | 1u64 << (square - 1) | 1u64 << (square - 2)) & attacked_squares == 0
        && (rook_moves(square, all_pieces, all_pieces)
            & rook_moves(square - 3, all_pieces, all_pieces))
        .count_ones()
            == 2
    {
        result |= 1 << ((square - File::from_square(square).to_u8()) + 1);
    };
    if queen_castle
        && (1u64 << square | 1u64 << (square + 1) | 1u64 << (square + 2)) & attacked_squares == 0
        && (rook_moves(square, all_pieces, all_pieces)
            & rook_moves(square + 4, all_pieces, all_pieces))
        .count_ones()
            == 3
    {
        result |= 1 << ((square - File::from_square(square).to_u8()) + 5);
    };
    result
}

fn king_attacked_rooks(king: u8, all_pieces: u64, rooks_and_queens: u64) -> u64 {
    let king_moves = rook_moves(king, rooks_and_queens, 0);
    let mut pinned_pieces = 0;
    for index in BitboardIterator::new(rooks_and_queens) {
        let moves = rook_moves(index, 1 << king, 0);
        if (moves & (king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
            if mask != 0 && masked_pieces.count_ones() == 0 {
                pinned_pieces |= 1 << index;
            }
        }
    }
    pinned_pieces
}

fn king_attacked_bishops(king: u8, all_pieces: u64, bishops_and_queens: u64) -> u64 {
    let king_moves = bishop_moves(king, bishops_and_queens, 0);
    let mut pinned_pieces = 0;
    for index in BitboardIterator::new(bishops_and_queens) {
        let moves = bishop_moves(index, 1 << king, 0);
        if (moves & (king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
            if mask != 0 && masked_pieces.count_ones() == 0 {
                pinned_pieces |= 1 << index;
            }
        }
    }
    pinned_pieces
}

fn king_attacked_knights(king: u8, enemy_knights: u64) -> u64 {
    knight_moves(king, 0) & enemy_knights
}

///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_rook(
    king: u8,
    friendly_pieces: u64,
    all_pieces: u64,
    rooks_and_queens: u64,
) -> u64 {
    let king_moves = rook_moves(king, rooks_and_queens, 0);
    let mut pinned_pieces = 0;
    for index in BitboardIterator::new(rooks_and_queens) {
        let moves = rook_moves(index, 1 << king, 0);
        if (moves & (king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
            if masked_pieces.count_ones() == 1
                && (masked_pieces & friendly_pieces).count_ones() == 1
            {
                pinned_pieces |= masked_pieces;
            }
        }
    }
    pinned_pieces
}

///returns a bitboard of all friendly pieces that are pinned to the king.
fn king_pins_bishop(
    king: u8,
    friendly_pieces: u64,
    all_pieces: u64,
    bishops_and_queens: u64,
) -> u64 {
    let mut new_bishops_and_queens = bishops_and_queens;
    let mut index = bishops_and_queens.trailing_zeros() as u8;
    let king_moves = bishop_moves(king, bishops_and_queens, 0);
    let mut pinned_pieces = 0;
    while index <= 63 && new_bishops_and_queens != 0 {
        new_bishops_and_queens = (bishops_and_queens >> index) & !1;
        let moves = bishop_moves(index, 1 << king, 0);
        if (moves & (king_moves | 1 << king)).count_ones() > 0 && (moves) & (1 << king) != 0 {
            let mask = moves & (king_moves | 1 << king);
            let masked_pieces = (all_pieces & !(1u64 << king)) & mask;
            if masked_pieces.count_ones() == 1
                && (masked_pieces & friendly_pieces).count_ones() == 1
            {
                pinned_pieces |= masked_pieces;
            }
        }
        index += new_bishops_and_queens.trailing_zeros() as u8;
    }
    pinned_pieces
}

fn knight_moves(square: u8, friendly_pieces: u64) -> u64 {
    crate::precomputed::moves::KNIGHT_MOVES[square as usize] & !friendly_pieces
}

fn bishop_moves(square: u8, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let magic = crate::precomputed::bishop_magic::BISHOP_MAGICS[square as usize];
    let blockers = all_pieces & crate::precomputed::moves::BISHOP_MASK[square as usize];
    let magic_index = crate::precomputed::magic_to_index::<9>(magic, blockers);
    crate::precomputed::bishop_magic::BISHOP_ATTACKS[square as usize][magic_index] & !friendly_pieces
}

fn rook_moves(square: u8, all_pieces: u64, friendly_pieces: u64) -> u64 {
    let magic = crate::precomputed::rook_magic::ROOK_MAGICS[square as usize];
    let blockers = all_pieces & crate::precomputed::moves::ROOK_MASK[square as usize];
    let magic_index = crate::precomputed::magic_to_index::<12>(magic, blockers);
    crate::precomputed::rook_magic::ROOK_ATTACKS[square as usize][magic_index] & !friendly_pieces
}

fn queen_moves(square: u8, all_pieces: u64, friendly_pieces: u64) -> u64 {
    bishop_moves(square, all_pieces, friendly_pieces)
        | rook_moves(square, all_pieces, friendly_pieces)
}

fn move_to_algebraic(move_: &Move, board: &BoardState) -> String {
    static SQUARES: [&str; 64] = [
        "h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1", "h2", "g2", "f2", "e2", "d2", "c2", "b2",
        "a2", "h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3", "h4", "g4", "f4", "e4", "d4", "c4",
        "b4", "a4", "h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5", "h6", "g6", "f6", "e6", "d6",
        "c6", "b6", "a6", "h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7", "h8", "g8", "f8", "e8",
        "d8", "c8", "b8", "a8",
    ];
    static FILES: [&str; 8] = ["h", "g", "f", "e", "d", "c", "b", "a"];
    static RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
    let file_mask = FILE_H << (File::from_square(move_.from).to_u8());
    let from_file = (File::from_square(move_.from).to_u8()) as usize;
    let rank_mask = RANK_1 << (move_.from / 8);
    let from_rank = (Rank::from_square(move_.from).to_u8()) as usize;
    let mut rank = "";
    let mut file = "";
    let mut promotion = "";
    let to_square = SQUARES[move_.to as usize];
    let piece_square = 1u64 << move_.from;
    let piece = if piece_square & board.black.rooks != 0 {
        if (board.black.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.black.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.black.bishops & bishop_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
        }
        "r"
    } else if piece_square & board.black.bishops != 0 {
        if (board.black.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.black.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.black.bishops & bishop_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
        }
        "b"
    } else if piece_square & board.black.knights != 0 {
        if (board.black.knights & knight_moves(move_.to, 0) & rank_mask).count_ones() > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.black.knights & knight_moves(move_.to, 0) & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.black.knights & knight_moves(move_.to, 0)).count_ones() > 1 {
            file = FILES[from_file];
        }
        "n"
    } else if piece_square & board.black.queens != 0 {
        if (board.black.queens & queen_moves(move_.to, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.black.queens
            & queen_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.black.queens & queen_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
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
            file = FILES[from_file];
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
        if (board.white.rooks & rook_moves(move_.to, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.white.rooks
            & rook_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.white.rooks & rook_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
        }
        "R"
    } else if piece_square & board.white.bishops != 0 {
        if (board.white.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.white.bishops
            & bishop_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.white.bishops & bishop_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
        }
        "B"
    } else if piece_square & board.white.knights != 0 {
        if (board.white.knights & knight_moves(move_.to, 0) & rank_mask).count_ones() > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.white.knights & knight_moves(move_.to, 0) & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.white.knights & knight_moves(move_.to, 0)).count_ones() > 1 {
            file = FILES[from_file];
        }
        "N"
    } else if piece_square & board.white.queens != 0 {
        if (board.white.queens & queen_moves(move_.to, board.all_pieces(), 0) & rank_mask)
            .count_ones()
            > 1
            && (board.black.queens
                & queen_moves(move_.to, board.all_pieces(), 0)
                & file_mask)
                .count_ones()
                > 1
        {
            file = FILES[from_file];
            rank = RANKS[from_rank];
        } else if (board.white.queens
            & queen_moves(move_.to, board.all_pieces(), 0)
            & file_mask)
            .count_ones()
            > 1
        {
            rank = RANKS[from_rank];
        } else if (board.white.queens & queen_moves(move_.to, board.all_pieces(), 0))
            .count_ones()
            > 1
        {
            file = FILES[from_file];
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
            file = FILES[from_file];
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
    let moves = board.generate_moves(&mut move_array, false);
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
    let moves = board.generate_moves(&mut move_array, false);
    if depth == 1 {
        for player_move in moves.iter().rev() {
            let move_string = move_to_algebraic(player_move, board);
            println!("{}: 1", move_string);
        }
        return moves.len() as u64;
    };
    let move_strings: Vec<_> = moves
        .iter()
        .rev()
        .map(|x| move_to_algebraic(x, board))
        .collect();
    for (i, player_move) in moves.iter().rev().enumerate() {
        print!("{}: ", move_strings[i]);
        board.make_move(*player_move);
        let result = perft(board, depth - 1);
        println!("{}", result);
        move_count += result;
        board.unmake_last_move();
    }
    move_count
}
