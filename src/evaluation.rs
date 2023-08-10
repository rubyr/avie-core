use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::board::PlayerState;
use crate::{
    board::{pawn_attacks, BitboardIterator, BoardState, Move, PieceType, Promotion},
    gamestate::Player,
};
use crate::precomputed::square_tables::{PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE, ROOK_TABLE, QUEEN_TABLE, KING_MIDGAME_TABLE, KING_ENDGAME_TABLE};

static BEST_SCORE: i64 = i64::MAX;
static WORST_SCORE: i64 = -i64::MAX;

static PAWN_SCORE: i64 = 100;
static KNIGHT_SCORE: i64 = 300;
static BISHOP_SCORE: i64 = 300;
static ROOK_SCORE: i64 = 500;
static QUEEN_SCORE: i64 = 900;
static KING_SCORE: i64 = 20000;
static SEARCH_WINDOW: i64 = 50;

fn reverse_bitboard(bitboard: u64, is_black: bool) -> u64 {
    if is_black {
        bitboard.reverse_bits()
    } else {
        bitboard
    }
}

fn position_score(player: &PlayerState, is_black: bool) -> i64 {
    BitboardIterator::new(reverse_bitboard(player.pawns, is_black)).fold(0i64, |init, pawn| init + PAWN_TABLE[pawn as usize])
    + BitboardIterator::new(reverse_bitboard(player.knights, is_black)).fold(0i64, |init, knight| init + KNIGHT_TABLE[knight as usize])
    + BitboardIterator::new(reverse_bitboard(player.bishops, is_black)).fold(0i64, |init, bishop| init + BISHOP_TABLE[bishop as usize])
    + BitboardIterator::new(reverse_bitboard(player.rooks, is_black)).fold(0i64, |init, rook| init + ROOK_TABLE[rook as usize])
    + BitboardIterator::new(reverse_bitboard(player.queens, is_black)).fold(0i64, |init, queen| init + QUEEN_TABLE[queen as usize])
    + BitboardIterator::new(reverse_bitboard(player.king, is_black)).fold(0i64, |init, king| init + KING_MIDGAME_TABLE[king as usize])
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
    let position_score = position_score(player, is_player_black) - position_score(opponent, !is_player_black);
    piece_score + position_score
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
    let mut scores: Vec<_> = moves.iter().map(|mov| move_score(board, mov)).collect();
    for i in 0..moves.len() {
        let mut j = i;
        while j > 0 && scores[j - 1] < scores[j] {
            scores.swap(j - 1, j);
            moves.swap(j - 1, j);
            j = j - 1;
        }
    }
    moves
}

fn alpha_beta_search(
    board: &mut BoardState,
    depth: i64,
    mut alpha: i64,
    beta: i64,
    should_stop: &AtomicBool,
) -> i64 {
    if should_stop.load(Ordering::Relaxed) {
        return alpha;
    }
    if depth <= 0 {
        return evaluate_position(board);
    }
    let mut move_data = [Move::default(); 218];
    let moves = board.generate_moves(&mut move_data);
    if moves.is_empty() {
        if board.is_in_check() {
            return WORST_SCORE;
        } else {
            return 0;
        }
    }
    sort_moves(board, moves);
    for mov in moves {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        board.make_move(*mov);
        let score = -alpha_beta_search(board, depth - 1, -beta, -alpha, should_stop);
        board.unmake_last_move();
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        if score > beta {
            return beta;
        }
        alpha = std::cmp::max(alpha, score);
    }
    return alpha;
}

pub fn choose_best_move(
    board: &mut BoardState,
    moves: &mut [Move],
    should_stop: &AtomicBool,
) -> (Move, i64) {
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
    let mut depth = 1;
    'search: while !should_stop.load(Ordering::Relaxed) {
        println!("info depth {}", depth);
        let since_start = std::time::Instant::now() - start_time;
        println!("info time {}", since_start.as_millis());
        for i in 0..moves.len() {
            if should_stop.load(Ordering::Relaxed) {
                break 'search;
            }
            let mut alpha;
            let mut beta;
            let mut alpha_window = SEARCH_WINDOW;
            let mut beta_window = SEARCH_WINDOW;
            if scores[i] == WORST_SCORE {
                alpha = WORST_SCORE;
                beta = BEST_SCORE;
            }
            else {
                alpha = scores[i] - alpha_window;
                beta = scores[i] + beta_window;
            }
            board.make_move(moves[i]);
            let mut score = -alpha_beta_search(board, depth, alpha, beta, should_stop);
            while score >= beta || score <= alpha {
                if score >= beta {
                    beta_window = beta_window.saturating_mul(4);
                    beta = scores[i] + beta_window;
                } else if score <= alpha {
                    alpha_window = alpha_window.saturating_mul(4);
                    alpha = scores[i] + alpha_window;
                }
               score =  -alpha_beta_search(board, depth, alpha, beta, should_stop);
            }
            scores[i] = score;
            board.unmake_last_move();
            if should_stop.load(Ordering::Relaxed) {
                break 'search;
            }
            let mut j = i;
            while j > 0 && scores[j - 1] < scores[j] {
                scores.swap(j - 1, j);
                moves.swap(j - 1, j);
                j = j - 1;
            }
        }
        depth += 1;
    }
    return (moves[0], scores[0]);
}
