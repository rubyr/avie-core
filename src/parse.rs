use nom::character::complete::{one_of, char};
use nom::combinator::opt;
use nom::sequence::{terminated};
use nom::IResult;
use nom::multi::{count, many1};

pub fn fen_to_game(input: &str) {
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
                //return Err();
            }
            else {
                board[num] = result_row[0..8].try_into().unwrap();
            }
        }
    }
    for row in board {
        println!("{}",String::from_iter(row.iter()));
    }
}

fn fen_board(input: &str) -> IResult<&str, Vec<Vec<char>>>{
    count(terminated(many1(one_of("rnbqkpRNBQKP12345678")), opt(char('/'))), 8)(input)
}