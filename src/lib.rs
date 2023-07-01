pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fen_starting_board() {
        use parse::*;
        let result = parse::fen_to_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(result, Ok(ParsedGameState{
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
            active_player: ActivePlayer::White,
            castling_rights: CastlingRights{
                black_kingside:true,
                black_queenside:true,
                white_kingside:true, 
                white_queenside:true
            },
            en_passant_target: None,
            half_turn_clock: 0,
            full_turn_clock: 1
    }));
        let result = parse::fen_to_game("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(result, Ok(ParsedGameState{
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
            active_player: ActivePlayer::Black,
            castling_rights: CastlingRights{
                black_kingside:true,
                black_queenside:true,
                white_kingside:true, 
                white_queenside:true
            },
            en_passant_target: Some((Rank::E, File::Three)),
            half_turn_clock: 0,
            full_turn_clock: 1
    }))
}
}
