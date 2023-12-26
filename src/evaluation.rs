use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::board::PlayerState;
use crate::precomputed::square_tables::{
    BISHOP_TABLE, KING_MIDGAME_TABLE, KNIGHT_TABLE, PAWN_TABLE, QUEEN_TABLE, ROOK_TABLE,
};
use crate::{
    board::{pawn_attacks, BitboardIterator, BoardState, Move, PieceType, Promotion},
    gamestate::Player,
};

static BEST_SCORE: i64 = i64::MAX;
static WORST_SCORE: i64 = -i64::MAX;

static PAWN_SCORE: i64 = 100;
static KNIGHT_SCORE: i64 = 300;
static BISHOP_SCORE: i64 = 300;
static ROOK_SCORE: i64 = 500;
static QUEEN_SCORE: i64 = 900;
static KING_SCORE: i64 = 20000;
pub enum ScoreType {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct MoveData {
    score: i64,
    depth: u64,
    score_type: ScoreType,
    _age: u64,
}

fn reverse_bitboard(bitboard: u64, is_black: bool) -> u64 {
    if is_black {
        bitboard.swap_bytes()
    } else {
        bitboard
    }
}

fn position_score(player: &PlayerState, is_black: bool) -> i64 {
    BitboardIterator::new(reverse_bitboard(player.pawns, is_black))
        .fold(0i64, |init, pawn| init + PAWN_TABLE[pawn as usize])
        + BitboardIterator::new(reverse_bitboard(player.knights, is_black))
            .fold(0i64, |init, knight| init + KNIGHT_TABLE[knight as usize])
        + BitboardIterator::new(reverse_bitboard(player.bishops, is_black))
            .fold(0i64, |init, bishop| init + BISHOP_TABLE[bishop as usize])
        + BitboardIterator::new(reverse_bitboard(player.rooks, is_black))
            .fold(0i64, |init, rook| init + ROOK_TABLE[rook as usize])
        + BitboardIterator::new(reverse_bitboard(player.queens, is_black))
            .fold(0i64, |init, queen| init + QUEEN_TABLE[queen as usize])
        + BitboardIterator::new(reverse_bitboard(player.king, is_black))
            .fold(0i64, |init, king| init + KING_MIDGAME_TABLE[king as usize])
}

fn piece_score(player: &PlayerState) -> i64 {
    let pawns = PAWN_SCORE * player.pawns.count_ones() as i64;
    let knights = KNIGHT_SCORE * player.knights.count_ones() as i64;
    let bishops = BISHOP_SCORE * player.bishops.count_ones() as i64;
    let rooks = ROOK_SCORE * player.rooks.count_ones() as i64;
    let queens = QUEEN_SCORE * player.queens.count_ones() as i64;
    let king = KING_SCORE * player.king.count_ones() as i64;
    king + queens + rooks + bishops + knights + pawns
}

fn evaluate_position(board: &mut BoardState) -> i64 {
    let player = board.player();
    let opponent = board.opponent();
    let is_player_black = board.active_player == Player::Black;
    let piece_score = piece_score(player) - piece_score(opponent);
    //let position_score =
    //    position_score(player, is_player_black) - position_score(opponent, !is_player_black);
    //piece_score + position_score
    piece_score
}

fn value_from_piece_type(piece: PieceType) -> i64 {
    match piece {
        PieceType::Pawn(_) => PAWN_SCORE,
        PieceType::Knight(_) => KNIGHT_SCORE,
        PieceType::Bishop(_) => BISHOP_SCORE,
        PieceType::Rook(_) => ROOK_SCORE,
        PieceType::Queen(_) => QUEEN_SCORE,
        PieceType::King(_) => KING_SCORE,
    }
}

fn move_score(board: &BoardState, mov: &Move) -> i64 {
    let mut result = 0;
    let our_piece = board
        .find_piece_on_square(mov.from)
        .expect("cannot move from empty square!");
    let their_piece = board.find_piece_on_square(mov.to);
    if let Some(piece) = their_piece {
        let their_score = value_from_piece_type(piece);
        let our_score = value_from_piece_type(our_piece);
        result += their_score - (our_score/10);
    }
    result += match mov.promotion {
        Promotion::None => 0,
        Promotion::Knight => value_from_piece_type(PieceType::Knight(Player::Black)),
        Promotion::Bishop => value_from_piece_type(PieceType::Bishop(Player::Black)),
        Promotion::Rook => value_from_piece_type(PieceType::Rook(Player::Black)),
        Promotion::Queen => value_from_piece_type(PieceType::Queen(Player::Black)),
    };
    let player = board.player();
    let opponent = board.opponent();
    let opponent_pawns = pawn_attacks(
        opponent.pawns,
        player.all_pieces(),
        board.en_passant_target,
        board.active_player,
    );
    if 1u64 << mov.from & (opponent_pawns[0] | opponent_pawns[1]) != 0 {
        result -= value_from_piece_type(our_piece);
    }
    result
}

fn sort_moves<'a>(board: &BoardState, moves: &'a mut [Move]) -> &'a mut [Move] {
    moves.sort_by_cached_key(|mov| move_score(board, mov));
    //let mut scores: Vec<_> = moves.iter().map(|mov| move_score(board, mov)).collect();
    //for i in 0..moves.len() {
    //    let mut j = i;
    //    while j > 0 && scores[j - 1] < scores[j] {
    //        scores.swap(j - 1, j);
    //        moves.swap(j - 1, j);
    //        j = j - 1;
    //    }
    //}
    moves
}

fn quiescence_search(
    board: &mut BoardState,
    nodes: &mut u64,
    mut alpha: i64,
    beta: i64,
    should_stop: &AtomicBool,
) -> i64 {
    todo!()
}

fn alpha_beta_search(
    board: &mut BoardState,
    depth: u64,
    nodes: &mut u64,
    mut alpha: i64,
    beta: i64,
    table: &mut HashMap<u64, MoveData>,
    should_stop: &AtomicBool,
) -> i64 {
    if should_stop.load(Ordering::Relaxed) {
        return alpha;
    }

    if depth == 0 {
        return evaluate_position(board);
    }
    let mut moves = [Move::default(); 218];
    let moves = board.generate_moves(&mut moves, false);
    if moves.is_empty() {
        if board.is_in_check() {
            return WORST_SCORE;
        }
        return 0; //draw
    }
    let mut best_score = WORST_SCORE;
    for mov in moves {
        #[cfg(debug_assertions)]
        let before = format!("{:?}", board);
        board.make_move(*mov);
        let score = alpha_beta_search(board, depth - 1, nodes, -beta, -alpha, table, should_stop);
        board.unmake_last_move();
        #[cfg(debug_assertions)]
        assert_eq!(before, format!("{:?}", board));
        best_score = std::cmp::max(best_score, score);
    }

    return alpha;
}

pub fn choose_best_move(
    board: &mut BoardState,
    moves: &mut [Move],
    table: &mut HashMap<u64, MoveData>,
    should_stop: &AtomicBool,
) -> (Move, i64) {
    let mut nodes = 0;
    let start_time = std::time::Instant::now();
    if moves.is_empty() {
        if board.is_in_check() {
            return (Move::default(), WORST_SCORE);
        } else {
            return (Move::default(), 0);
        }
    }
    sort_moves(board, moves);
    let mut scores = vec![WORST_SCORE; moves.len()];
    let mut nodes = 0;
    let depth = 4;
    let mut best_score = WORST_SCORE;
    let mut best_score_index = 0;
    for (i, mov) in moves.iter().enumerate() {
        board.make_move(*mov);
        scores[i] = alpha_beta_search(board, depth, &mut nodes, BEST_SCORE, WORST_SCORE, table, should_stop);
        board.unmake_last_move();
        if scores[i] > best_score {
            best_score = scores[i];
            best_score_index = i;
        }
    }
    println!("info depth 4 nodes {} time {}",
        nodes,
        (std::time::Instant::now() - start_time).as_millis()
    );
    return (moves[best_score_index], scores[best_score_index]);
}
