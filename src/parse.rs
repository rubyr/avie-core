use nom::character::complete::{one_of, char, multispace0};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::{recognize, opt};
use nom::sequence::{tuple, terminated, preceded};
use nom::IResult;
use nom::multi::{count, many1};

pub enum FenError<'a> {
    ParseErr(nom::Err<nom::error::Error<&'a str>>),
    InvalidRow(Vec<char>),
    InvalidActivePlayer(char),
}

pub struct ParsedGameState {
    piece_position: [[char;8];8],
    active_player: ActivePlayer,
    castling_rights: CastlingRights,
    en_passant: Option<(Rank, File)>,
    half_turn_clock: u8,
    full_turn_clock: u32
}

pub struct CastlingRights {
    black_kingside: bool,
    black_queenside: bool,
    white_kingside: bool,
    white_queenside:bool
}
#[repr(u8)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}
#[repr(u8)]
pub enum File {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

pub fn fen_to_game(input: &str) -> Result<(), FenError>{
    match tuple((fen_board, preceded(multispace0, fen_active_player), preceded(multispace0, fen_castling)))(input) {
        Err(e) => return Err(FenError::ParseErr(e)),
        Ok((_, (parsed_board, parsed_active_player, parsed_castling_rights))) => {
            let mut board: [[char;8];8] = [['.';8];8];
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
                    board[num] = result_row[0..8].try_into().unwrap();
                }
            }
            let active_player = match parsed_active_player {
                'b' => ActivePlayer::Black,
                'w' => ActivePlayer::White,
                x => return Err(FenError::InvalidActivePlayer(x))
            };
        }
    }
    
    if let Ok((rest, board_parse)) = fen_board(input) {
        
        if let Ok((rest, active_player)) = preceded(multispace0, fen_active_player)(rest) {
            
        }
    }
    todo!()
}

pub enum ActivePlayer {
    Black,
    White
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