#[cfg(test)]
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
        en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
        full_counter: 1,
        half_counter: 0,
        move_stack: vec![],
    };
    let king_move = king_moves(board.white.king.ilog2() as u8, board.white.all_pieces());
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
        en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
        full_counter: 1,
        half_counter: 0,
        move_stack: vec![],
    };
    let king_move = king_moves(board.white.king.ilog2() as u8, board.white.all_pieces());
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
        en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
        full_counter: 1,
        half_counter: 0,
        move_stack: vec![],
    };
    let king_move = king_moves(board.white.king.ilog2() as u8, board.white.all_pieces());
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
        move_stack: vec![],
    };
    let king_move = king_moves(board.white.king.ilog2() as u8, board.white.all_pieces());
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
        en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
        full_counter: 1,
        half_counter: 0,
        move_stack: vec![],
    };
    let mut new_knights = board.white.knights;
    let mut index = new_knights.trailing_zeros();
    let friendly_pieces = board.white.all_pieces();
    let mut moves = vec![];
    while index <= 63 && new_knights != 0 {
        new_knights = (new_knights >> index) & !1;
        moves.push(knight_moves(index as u8, friendly_pieces));
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
        move_stack: vec![],
    };
    let queen_move = queen_moves(
        board.white.queens.ilog2() as u8,
        board.all_pieces(),
        board.white.all_pieces(),
    );
    assert_eq!(queen_move, 0x0000000102040800);
}

#[test]
fn boardstate_new() {
use crate::gamestate::*;
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
            en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
            half_counter: 0,
            full_counter: 1,
            move_stack: vec![]
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
            full_counter: 1,
            move_stack: vec![]
        }
    );
}

#[test]
fn king_rook_pins() {
    let king = 0x0000000008000000u64;
    let rooks = 0x0800000081000000u64;
    let enemy = 0x08000000C1000000u64;
    let friendly = 0x000800001C000000u64;
    let result = king_pins_rook(king.ilog2() as u8, friendly, enemy | friendly, rooks);
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
    let result = king_pins_bishop(king.ilog2() as u8, friendly, enemy | friendly, bishops);
    assert_eq!(result, 0x0000001000000200);
}

#[test]
fn first_turn_legal_moves() {
    let mut move_array = [Move {
        from: 0,
        to: 0,
        promotion: Promotion::None,
    }; 218];
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
        en_passant_target: EnPassantTarget(EN_PASSANT_NO_SQUARE),
        half_counter: 0,
        full_counter: 1,
        move_stack: vec![],
    };
    let moves = board.generate_moves(&mut move_array);
    let x = moves.len();
    assert_eq!(x, 20);
}

#[test]
fn perft_test() {
    use crate::parse::fen_to_game;
    let parsed_state =
        fen_to_game("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ")
            .unwrap();
    let mut board = BoardState::new(parsed_state);
    let first_3_ply_moves = perft_div(&mut board, 6);
    //todo!() still 30 extra moves to figure out
    println!("{}", first_3_ply_moves);
    //assert_eq!(first_3_ply_moves, 6923051137);
}
