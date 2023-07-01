use nom::character::complete::{one_of, char, multispace1, u8, u64};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::{recognize, opt};
use nom::sequence::{tuple, terminated, preceded, pair};
use nom::IResult;
use nom::multi::{count, many1};

#[derive(PartialEq, Debug)]
#[allow(dead_code)]

pub enum FenError<'a> {
    ParseErr(nom::Err<nom::error::Error<&'a str>>),
    InvalidRow(Vec<char>),
    InvalidActivePlayer(char),
    InvalidRank(u8),
    InvalidFile(u8)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParsedGameState {
    pub piece_position: [[char;8];8],
    pub active_player: ActivePlayer,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<(Rank, File)>,
    pub half_turn_clock: u8,
    pub full_turn_clock: u64
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ActivePlayer {
    Black,
    White
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights {
    pub black_kingside: bool,
    pub black_queenside: bool,
    pub white_kingside: bool,
    pub white_queenside: bool
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Rank {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

impl TryFrom<u8> for Rank {
    type Error = u8;
    fn try_from(rank: u8) -> Result<Rank, Self::Error>{
        match rank {
            0..=7 => Ok(unsafe{std::mem::transmute(rank)}),
            x => Err(x)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum File {
    One   = 0,
    Two   = 1,
    Three = 2,
    Four  = 3,
    Five  = 4,
    Six   = 5,
    Seven = 6,
    Eight = 7
}

impl TryFrom<u8> for File {
    type Error = u8;
    fn try_from(rank: u8) -> Result<File, Self::Error>{
        match rank {
            0..=7 => Ok(unsafe{std::mem::transmute(rank)}),
            x => Err(x)
        }
    }
}

pub fn fen_to_game(input: &str) -> Result<ParsedGameState, FenError>{
    match tuple((fen_board, preceded(multispace1, fen_active_player), preceded(multispace1, fen_castling), preceded(multispace1, fen_en_passant_target), preceded(multispace1, u8), preceded(multispace1, u64)))(input) {
        Err(e) => return Err(FenError::ParseErr(e)),
        Ok((_, (parsed_board, parsed_active_player, parsed_castling_rights, parsed_en_passant_target, half_turn_clock, full_turn_clock))) => {
            let mut piece_position: [[char;8];8] = [['.';8];8];
            for (num, row) in parsed_board.into_iter().enumerate(){
                let mut result_row: Vec<char> = vec![];
                for c in row {
                    match c {
                        '1'..='8' => {
                            let x = (c as u8) - b'0';
                            result_row.append(&mut vec!['.'; x.into()]);
                        }
                        x => result_row.push(x)
                    };
                }
                if result_row.len() != 8 {
                    return Err(FenError::InvalidRow(result_row));
                }
                else {
                    piece_position[num] = result_row[0..8].try_into().unwrap();
                }
            }
            let active_player = match parsed_active_player {
                'b' => ActivePlayer::Black,
                'w' => ActivePlayer::White,
                x => return Err(FenError::InvalidActivePlayer(x))
            };
            let black_kingside = parsed_castling_rights.contains('k');
            let black_queenside = parsed_castling_rights.contains('q');
            let white_kingside = parsed_castling_rights.contains('K');
            let white_queenside = parsed_castling_rights.contains('Q');
            let castling_rights = CastlingRights{black_kingside, black_queenside, white_kingside, white_queenside};
            let en_passant_target = match parsed_en_passant_target {
                "-" => None,
                x => {
                    let rank = match Rank::try_from(x.as_bytes()[0] - b'a'){
                        Ok(rank) => rank,
                        Err(x) => return Err(FenError::InvalidRank(x))
                    };
                    let file = match File::try_from(x.as_bytes()[0] - b'a'){
                        Ok(file) => file,
                        Err(x) => return Err(FenError::InvalidFile(x))
                    };
                    Some((rank, file))
                }
            };
            Ok(ParsedGameState{piece_position, active_player, castling_rights, en_passant_target, half_turn_clock, full_turn_clock })
        }
    }
}

fn fen_board(input: &str) -> IResult<&str, Vec<Vec<char>>>{
    count(terminated(many1(one_of("rnbqkpRNBQKP12345678")), opt(char('/'))), 8)(input)
}

fn fen_active_player(input: &str) -> IResult<&str, char> {
    one_of("wb")(input)
}

fn fen_castling(input: &str) -> IResult<&str, &str> {
    alt((tag("-"),recognize(many1(one_of("KQkq")))))(input)
}

fn fen_en_passant_target(input: &str) -> IResult<&str, &str> {
    alt((tag("-"), recognize(pair(one_of("abcdefgh"), one_of("12345678")))))(input)
}