use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace1, one_of, u64, u8};
use nom::combinator::{opt, recognize};
use nom::multi::{count, many1};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

#[test]
fn fen_starting_board() {
    let result = fen_to_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(
        result,
        Ok(ParsedGameState {
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
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true
            },
            en_passant_target: None,
            half_turn_clock: 0,
            full_turn_clock: 1
        })
    );
}

#[test]
fn fen_first_move() {
    let result = fen_to_game("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    assert_eq!(
        result,
        Ok(ParsedGameState {
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
            castling_rights: CastlingRights {
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true
            },
            en_passant_target: Some((Rank::E, File::Three)),
            half_turn_clock: 0,
            full_turn_clock: 1
        })
    )
}

#[derive(PartialEq, Debug)]

pub enum FenError<'a> {
    ParseErr(nom::Err<nom::error::Error<&'a str>>),
    InvalidPiece(char),
    InvalidRow(Vec<char>),
    InvalidActivePlayer(char),
    InvalidPosition,
    InvalidRank(char),
    InvalidFile(char),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParsedGameState {
    pub piece_position: [[char; 8]; 8],
    pub active_player: ActivePlayer,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<(Rank, File)>,
    pub half_turn_clock: u8,
    pub full_turn_clock: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActivePlayer {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights {
    pub black_kingside: bool,
    pub black_queenside: bool,
    pub white_kingside: bool,
    pub white_queenside: bool,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Rank {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl TryFrom<char> for Rank {
    type Error = char;
    fn try_from(rank: char) -> Result<Rank, Self::Error> {
        match rank {
            'a'..='h' => Ok(unsafe { std::mem::transmute((rank as u8) - b'a') }),
            x => Err(x),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum File {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

impl TryFrom<char> for File {
    type Error = char;
    fn try_from(file: char) -> Result<File, Self::Error> {
        match file {
            '1'..='8' => Ok(unsafe { std::mem::transmute((file as u8) - b'1') }),
            x => Err(x),
        }
    }
}

fn fen_board(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    count(
        terminated(many1(one_of("rnbqkpRNBQKP12345678")), opt(char('/'))),
        8,
    )(input)
}

fn fen_active_player(input: &str) -> IResult<&str, char> {
    one_of("wb")(input)
}

fn fen_castling(input: &str) -> IResult<&str, &str> {
    alt((tag("-"), recognize(many1(one_of("KQkq")))))(input)
}

fn fen_en_passant_target(input: &str) -> IResult<&str, &str> {
    alt((
        tag("-"),
        recognize(pair(one_of("abcdefgh"), one_of("12345678"))),
    ))(input)
}

fn calculate_piece_position(
    parsed_board: Vec<Vec<char>>,
) -> Result<[[char; 8]; 8], FenError<'static>> {
    let mut piece_position = [['.'; 8]; 8];
    for (num, row) in parsed_board.into_iter().enumerate() {
        let mut result_row: Vec<char> = vec![];
        for c in row {
            match c {
                '1'..='9' => {
                    let x: u8 = c.to_digit(10).unwrap() as u8;
                    result_row.append(&mut vec!['.'; x.into()]);
                }
                'r' | 'n' | 'b' | 'q' | 'k' | 'p' | 'R' | 'N' | 'B' | 'Q' | 'K' | 'P' => {
                    result_row.push(c)
                }
                x => return Err(FenError::InvalidPiece(x)),
            };
        }
        if result_row.len() != 8 {
            return Err(FenError::InvalidRow(result_row));
        } else {
            piece_position[num] = result_row[0..8].try_into().unwrap();
        }
    }
    Ok(piece_position)
}

fn calculate_en_passant_target(
    parsed_en_passant_target: &str,
) -> Result<Option<(Rank, File)>, FenError> {
    match parsed_en_passant_target {
        "-" => Ok(None),
        x => {
            let rank = match x.chars().nth(0).map(Rank::try_from) {
                None => return Err(FenError::InvalidPosition),
                Some(x) => match x {
                    Ok(rank) => rank,
                    Err(x) => return Err(FenError::InvalidRank(x)),
                },
            };
            let file = match x.chars().nth(1).map(File::try_from) {
                None => return Err(FenError::InvalidPosition),
                Some(x) => match x {
                    Ok(file) => file,
                    Err(x) => return Err(FenError::InvalidFile(x)),
                },
            };
            Ok(Some((rank, file)))
        }
    }
}

pub fn fen_to_game(input: &str) -> Result<ParsedGameState, FenError> {
    match tuple((
        fen_board,
        preceded(multispace1, fen_active_player),
        preceded(multispace1, fen_castling),
        preceded(multispace1, fen_en_passant_target),
        preceded(multispace1, u8),
        preceded(multispace1, u64),
    ))(input)
    {
        Err(e) => return Err(FenError::ParseErr(e)),
        Ok((
            _,
            (
                parsed_board,
                parsed_active_player,
                parsed_castling_rights,
                parsed_en_passant_target,
                half_turn_clock,
                full_turn_clock,
            ),
        )) => {
            let piece_position: [[char; 8]; 8] = calculate_piece_position(parsed_board)?;
            let active_player = match parsed_active_player {
                'b' => ActivePlayer::Black,
                'w' => ActivePlayer::White,
                x => return Err(FenError::InvalidActivePlayer(x)),
            };
            let black_kingside = parsed_castling_rights.contains('k');
            let black_queenside = parsed_castling_rights.contains('q');
            let white_kingside = parsed_castling_rights.contains('K');
            let white_queenside = parsed_castling_rights.contains('Q');
            let castling_rights = CastlingRights {
                black_kingside,
                black_queenside,
                white_kingside,
                white_queenside,
            };
            let en_passant_target = calculate_en_passant_target(parsed_en_passant_target)?;
            Ok(ParsedGameState {
                piece_position,
                active_player,
                castling_rights,
                en_passant_target,
                half_turn_clock,
                full_turn_clock,
            })
        }
    }
}
