use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::board::move_to_algebraic;
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
    let position_score =
        position_score(player, is_player_black) - position_score(opponent, !is_player_black);
    piece_score //+ position_score
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
        result += 10 * their_score - our_score;
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
    moves.sort_by_cached_key(|mov| -move_score(board, mov)); //want to have highest scores first
    moves
}

fn quiescence_search(
    board: &mut BoardState,
    nodes: &mut u64,
    mut alpha: i64,
    beta: i64,
    should_stop: &AtomicBool,
) -> i64 {
    //stand_pat should be skipped if in lategame
    let stand_pat = evaluate_position(board);
    let mut moves = [Move::default(); 218];
    let moves = board.generate_moves(&mut moves, true);
    sort_moves(board, moves);
    for mov in moves {
        *nodes += 1;
        board.make_move(*mov);
        let score = -quiescence_search(board, nodes, alpha, beta, should_stop);
        board.unmake_last_move();
        if score >= beta {
            return beta;
        }
        alpha = std::cmp::max(alpha, score);
    }
    std::cmp::max(stand_pat, alpha)
}

fn search(
    board: &mut BoardState,
    depth: u64,
    nodes: &mut u64,
    mut alpha: i64,
    beta: i64,
    table: &mut HashMap<u64, MoveData>,
    should_stop: &AtomicBool,
) -> i64 {
    if should_stop.load(Ordering::Relaxed) {
        return WORST_SCORE;
    }

    if depth == 0 {
        return quiescence_search(board, nodes, -beta, -alpha, should_stop);
    }
    let mut moves = [Move::default(); 218];
    let moves = board.generate_moves(&mut moves, false);
    if moves.is_empty() {
        if board.is_in_check() {
            return WORST_SCORE;
        }
        return 0; //draw
    }
    sort_moves(board, moves);
    for mov in moves {
        *nodes += 1;
        board.make_move(*mov);
        let score = -search(board, depth - 1, nodes, -beta, -alpha, table, should_stop);
        board.unmake_last_move();
        if score >= beta {
            return beta;
        }
        alpha = std::cmp::max(alpha, score);
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
    let mut nodes = 0;
    let mut depth = 0;
    let mut best_score = WORST_SCORE;
    sort_moves(board, moves);
    while !should_stop.load(Ordering::Relaxed) {
        depth += 1;
        for i in 0..moves.len() {
            nodes += 1;
            board.make_move(moves[i]);
            let score = -search(
                board,
                depth - 1,
                &mut nodes,
                WORST_SCORE,
                BEST_SCORE,
                table,
                should_stop,
            );
            board.unmake_last_move();
            if should_stop.load(Ordering::Relaxed) {
                break;
            }
            if score > best_score {
                println!("depth: {}, new best move: {}, new score: {}, old score: {}", depth, move_to_algebraic(&moves[i], board), score, best_score);
                best_score = score;
                //ensures that the first move of the principal variation is always in index 0
                //mandatory for iterative deepening to produce correct results
                //unsure if this is the best solution considering that larger upsets will push previously good moves
                //very far back in the search, potentially causing it to repeatedly move the same two moves back and forth
                //at alternating depths.
                //however, if the move ordering heuristic is high quality, this should be rare.
                moves.swap(0, i);
            }
        }
    }
    println!(
        "info depth {} nodes {} time {}",
        depth,
        nodes,
        (std::time::Instant::now() - start_time).as_millis()
    );

    for i in 0..moves.len() {
        print!(
            "{}: {}, ",
            move_to_algebraic(&moves[i], board),
            -move_score(board, &moves[i])
        );
    }
    return (moves[0], best_score);
}
