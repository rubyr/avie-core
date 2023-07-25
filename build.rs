static FILE_A: u64 = 0x8080808080808080u64;
static FILE_B: u64 = 0x4040404040404040u64;
static FILE_G: u64 = 0x0202020202020202u64;
static FILE_H: u64 = 0x0101010101010101u64;
static NOT_FILE_A: u64 = !FILE_A;
static NOT_FILE_B: u64 = !FILE_B;
static NOT_FILE_G: u64 = !FILE_G;
static NOT_FILE_H: u64 = !FILE_H;
static NOT_FILE_AB: u64 = NOT_FILE_A & NOT_FILE_B;
static NOT_FILE_GH: u64 = NOT_FILE_G & NOT_FILE_H;
static RANK_1: u64 = 0x00000000000000FFu64;
static RANK_8: u64 = 0xFF00000000000000u64;
static MAIN_DIAGONAL: u64 = 0x8040201008040201u64;
static ANTIMAIN_DIAGONAL: u64 = 0x0102040810204080u64;

use std::io::Write;

pub fn generate_knight_moves() -> String {
    let mut moves = [0u64; 64];
    for i in 0..64 {
        let knights = 1 << i;
        let mut current_move = [0u64; 8];
        current_move[0] = (knights << 17) & NOT_FILE_H;
        current_move[1] = (knights << 10) & NOT_FILE_GH;
        current_move[2] = (knights >> 6) & NOT_FILE_GH;
        current_move[3] = (knights >> 15) & NOT_FILE_H;
        current_move[7] = (knights << 15) & NOT_FILE_A;
        current_move[6] = (knights << 6) & NOT_FILE_AB;
        current_move[5] = (knights >> 10) & NOT_FILE_AB;
        current_move[4] = (knights >> 17) & NOT_FILE_A;
        moves[i] = current_move.iter().fold(0, |x, y| x | y);
    }
    let mut string = String::new();
    string.push_str("[\n");
    let mut first = true;
    for i in moves {
        if first == true {
            first = false;
        } else {
            string.push_str(", \n")
        }
        string.push_str(format!("0x{:016X}u64", i).as_str());
    }
    string.push_str("];\n");
    string
}

pub fn generate_king_moves() -> String {
    let mut moves = [0u64; 64];
    for i in 0..64 {
        let king = 1 << i;
        moves[i] = ((king << 9 | king << 1 | king >> 7) & NOT_FILE_H)
            | king << 8
            | ((king << 7 | king >> 1 | king >> 9) & NOT_FILE_A)
            | king >> 8;
    }
    let mut string = String::new();
    string.push_str("[\n");
    let mut first = true;
    for i in moves {
        if first == true {
            first = false;
        } else {
            string.push_str(", \n")
        }
        string.push_str(format!("0x{:016X}u64", i).as_str());
    }
    string.push_str("];\n");
    string
}

pub fn generate_rook_mask() -> [u64; 64] {
    let mut mask = [0u64; 64];
    let mut file_mask = [0u64; 8];
    let mut rank_mask = [0u64; 8];
    for i in 0..8 {
        file_mask[i] = FILE_H << i;
        rank_mask[i] = RANK_1 << i * 8;
    }
    let mut string = String::new();
    string.push_str("[");
    let mut first = true;
    for i in 0..64 {
        let mut bitmask = 0u64;
        if i / 8 != 7 {
            bitmask |= RANK_8
        };
        if i / 8 != 0 {
            bitmask |= RANK_1
        };
        if i % 8 != 7 {
            bitmask |= FILE_A
        };
        if i % 8 != 0 {
            bitmask |= FILE_H
        };
        mask[i] = (file_mask[i % 8] | rank_mask[i / 8]) & !(1 << i); //& !bitmask;
    }
    mask
}

pub fn generate_bishop_mask() -> [u64; 64] {
    let mut mask = [0u64; 64];
    let mut string = String::new();
    string.push_str("[");
    let mut first = true;
    for i in 0..64isize {
        let diag = 8 * (i & 7) - (i & 56);
        let diag_north = -diag & (diag >> 31);
        let diag_south = diag & (-diag >> 31);
        let anti_diag = 56 - 8 * (i & 7) - (i & 56);
        let anti_diag_north = -anti_diag & (anti_diag >> 31);
        let anti_diag_south = anti_diag & (-anti_diag >> 31);
        mask[i as usize] = (((MAIN_DIAGONAL as u64 >> diag_south) << diag_north)
            | ((ANTIMAIN_DIAGONAL as u64 >> anti_diag_south) << anti_diag_north))
            & !(1 << i);
    };
    mask
}

#[derive(PartialEq, Eq)]
enum Piece {Rook, Bishop}

impl Piece {
    fn moves(&self, square: u64, blockers: u64) -> u64{
        let up_blocked = blockers | RANK_8;
        let down_blocked = blockers | RANK_1;
        let west_blocked = blockers | FILE_A;
        let east_blocked = blockers | FILE_H;
        let up_east_blocked = up_blocked | east_blocked;
        let up_west_blocked = up_blocked | west_blocked;
        let down_west_blocked = down_blocked | west_blocked;
        let down_east_blocked = down_blocked | east_blocked;
        let piece_square = 1u64 << square;
        let mut valid_moves = 0u64;
        if *self == Piece::Rook {
            let mut up_dir = 0;
            let mut down_dir = 0;
            let mut west_dir = 0;
            let mut east_dir = 0;
            'up: loop {
                if (piece_square << (8 * up_dir)) & !up_blocked == 0 {
                    break 'up;
                }
                up_dir += 1;
                valid_moves |= piece_square << (8 * up_dir);
            }
            'down: loop {
                if (piece_square >> (8 * down_dir)) & !down_blocked == 0 {
                    break 'down;
                }
                down_dir += 1;
                valid_moves |= piece_square >> (8 * down_dir);
            }
            'west: loop {
                if (piece_square << west_dir) & !west_blocked == 0 {
                    break 'west;
                }
                west_dir += 1;
                valid_moves |= piece_square << west_dir;
            }
            'east: loop {
                if (piece_square >> east_dir) & !east_blocked == 0 {
                    break 'east;
                }
                east_dir += 1;
                valid_moves |= piece_square >> east_dir;
            }
        }
        else {
            let mut up_east_dir = 0;
            let mut up_west_dir = 0;
            let mut down_east_dir = 0;
            let mut down_west_dir = 0;
            'up_east: loop {
                if (piece_square << (7 * up_east_dir)) & !up_east_blocked == 0 {
                    break 'up_east;
                }
                up_east_dir += 1;
                valid_moves |= piece_square << (7 * up_east_dir);
            }
            'up_west: loop {
                if (piece_square << (9 * up_west_dir)) & !up_west_blocked == 0 {
                    break 'up_west;
                }
                up_west_dir += 1;
                valid_moves |= piece_square << (9 * up_west_dir);
            }
            'down_west: loop {
                if (piece_square >> (7* down_west_dir)) & !down_west_blocked == 0 {
                    break 'down_west;
                }
                down_west_dir += 1;
                valid_moves |= piece_square >> (7 * down_west_dir);
            }
            'down_east: loop {
                if (piece_square >> (9 * down_east_dir)) & !down_east_blocked == 0 {
                    break 'down_east;
                }
                down_east_dir += 1;
                valid_moves |= piece_square >> (9 *down_east_dir);
            }
        }
        valid_moves
    }
}

pub fn generate_rook_magics(mask: &[u64; 64]) -> String{
    let mut array: Vec<(u64, Vec<u64>)> = vec![(0, vec![]);64];
    let bits = 12;
    let mut found = 0;
    for square in 0..64 {
        't: loop {
            let magic = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
            let rook_mask = mask[square as usize];
            if let Ok(table) = generate_table(magic, rook_mask, bits, square, Piece::Rook) {
                println!("index: {:?}", square);
                array[square as usize] = (magic, table);
                found += 1;
                println!("found {} rook magics", found);
                //println!("(magic: {:?}, bits: {:?}, table: {:?})", magic, bits, table);
                break 't;
            }
        }
    }
    let mut string = format!("pub static ROOK_MAGICS : [u64;64] = [");
    let mut string2 = format!("pub static ROOK_ATTACKS: [[u64;{:?}];64] = [", 1<<bits);
    let mut first = true;
    for (magic, list) in array {
        if first == true {
            first = false;
        }
        else {
            string.push_str(", \n");
            string2.push_str(", \n");
        }
        string.push_str(&mut format!("\t0x{:016X}", magic));
        string2.push_str("\t[\n");
        let mut first_list = true;
        for i in list {
            if first_list == true {
                first_list = false;
            }
            else {
                string2.push_str(", \n");
            }
            string2.push_str(&mut format!("\t\t0x{:016X}", i));
        }
        string2.push_str("\n\t]");
    };
    string.push_str("\n];\n");
    string2.push_str("\n];");
    string.push_str(string2.as_str());
    string
}

pub fn generate_bishop_magics(mask: &[u64; 64]) -> String {
    let mut array: Vec<(u64, Vec<u64>)> = vec![(0, vec![]);64];
    let bits = 9;
    let mut found = 0;
    for square in 0..64 {
        't: loop {
            let magic = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
            let bishop_mask = mask[square as usize];
            if let Ok(table) = generate_table(magic, bishop_mask, bits, square, Piece::Bishop) {
                println!("index: {:?}", square);
                array[square as usize] = (magic, table);
                found += 1;
                println!("found {} bishop magics", found);
                //println!("(magic: {:?}, bits: {:?}, table: {:?})", magic, bits, table);
                break 't;
            }
        }
    }
    let mut string = format!("pub static BISHOP_MAGICS : [u64;64] = [\n");
    let mut string2 = format!("pub static BISHOP_ATTACKS: [[u64;512];64] = [\n");
    let mut first = true;
    for (magic, list) in array {
        if first == true {
            first = false;
        }
        else {
            string.push_str(", \n");
            string2.push_str(", \n");
        }
        string.push_str(&mut format!("\t0x{:016X}", magic));
        string2.push_str("\t[\n");
        let mut first_list = true;
        for i in list {
            if first_list == true {
                first_list = false;
            }
            else {
                string2.push_str(", \n");
            }
            string2.push_str(&mut format!("\t\t0x{:016X}", i));
        }
        string2.push_str("\n\t]");
    };
    string.push_str("\n];\n");
    string2.push_str("\n];");
    string.push_str(string2.as_str());
    string
}

fn generate_table(magic: u64, piece_mask: u64, bits: u64, square: u64, piece: Piece) -> Result<Vec<u64>, ()> {
    let mut table = vec![0u64; 1 << bits];
    for permutation in permutations(piece_mask) {
        let moves = piece.moves(square, permutation);
        let entry = &mut table[magic_to_index(magic, permutation, bits)];
        if *entry == 0 {
            *entry = moves;
        }
        else if *entry != moves{
            return Err(());
        }
    }
    Ok(table)
}

fn permutations(piece_mask: u64) -> Vec<u64> {
    let mut bits = vec![];
    for i in 0..64u8 {
        if (piece_mask >> i) & 1 == 1 {
            bits.push(i);
        }
    }
    let permutation_count: u64 = 1 << piece_mask.count_ones();
    let mut permutations = vec![0u64; permutation_count as usize];
    for j in 0u64..permutation_count {
        for k in 0u64..bits.len() as u64 {
            let bit = (j >> k) & 1;
            permutations[j as usize] |= bit << bits[k as usize];
        }
    }
    permutations
}

pub fn magic_to_index(magic: u64, permutation: u64, bits: u64) -> usize {
    (permutation.wrapping_mul(magic) >> (64 - bits)) as usize
}

fn main() {
    let knight_moves = generate_knight_moves();
    let king_moves = generate_king_moves();

    let mut move_file = std::fs::File::create("src/precomputed/precomputed.rs").unwrap();
    let rook_mask = generate_rook_mask();
    let bishop_mask = generate_bishop_mask();
    move_file.write("pub static KNIGHT_MOVES: [u64; 64] = ".as_bytes()).unwrap();
    move_file.write(knight_moves.as_bytes()).unwrap();
    move_file.write("pub static KING_MOVES: [u64; 64] = ".as_bytes()).unwrap();
    move_file.write(king_moves.as_bytes()).unwrap();
    move_file.write("pub static ROOK_MASK: [u64; 64] = ".as_bytes()).unwrap();
    move_file.write(format!("{:#?};", rook_mask).as_bytes()).unwrap();
    move_file.write("pub static BISHOP_MASK: [u64; 64] = ".as_bytes()).unwrap();
    move_file.write(format!("{:#?};", bishop_mask).as_bytes()).unwrap();
    //let bishop_magics = generate_bishop_magics(&bishop_mask);
    //let rook_magics = generate_rook_magics(&rook_mask);
    //let mut bishop_magic_file = std::fs::File::create("src/precomputed/bishop_magic.rs").unwrap();
    //bishop_magic_file.write(bishop_magics.as_bytes()).unwrap();
    //let mut rook_magic_file = std::fs::File::create("src/precomputed/rook_magic.rs").unwrap();
    //rook_magic_file.write(rook_magics.as_bytes()).unwrap();
}