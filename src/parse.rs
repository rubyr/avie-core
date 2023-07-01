use nom::character::complete::{one_of, char, multispace0};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::{recognize, opt};
use nom::sequence::{terminated, preceded};
use nom::IResult;
use nom::multi::{count, many1};

pub fn fen_to_game(input: &str) -> Result<(), ()>{
    let mut board: [[char;8];8] = [['.';8];8];
    if let Ok((rest, board_parse)) = fen_board(input) {
        for (num, row) in board_parse.into_iter().enumerate(){
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
                return Err(());
            }
            else {
                board[num] = result_row[0..8].try_into().unwrap();
            }
        }
        if let Ok((rest, active_player)) = preceded(multispace0, fen_active_player)(rest) {
            let player = match active_player {
                'b' => ActivePlayer::Black,
                'w' => ActivePlayer::White,
                _ => return Err(())
            };
        }
    }

    for row in board {
        println!("{}",String::from_iter(row.iter()));
    };
    todo!()
}

enum ActivePlayer {
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