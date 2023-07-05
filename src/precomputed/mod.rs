mod rook_magic;
mod bishop_magic;
mod test {
    use std::io::Write;
    #[test]
    fn generate_knight_moves() {
        let mut moves = [0u64; 64];
        for i in 0..64 {
            let knights = 1 << i;
            let mut current_move = [0u64; 8];
            let not_a_file = 0x7F7F7F7F7F7F7F7Fu64;
            let not_b_file = 0xBFBFBFBFBFBFBFBFu64;
            let not_g_file = 0xFDFDFDFDFDFDFDFDu64;
            let not_h_file = 0xFEFEFEFEFEFEFEFEu64;
            let not_ab_file = not_a_file & not_b_file;
            let not_gh_file = not_g_file & not_h_file;
            current_move[0] = (knights << 17) & not_h_file;
            current_move[1] = (knights << 10) & not_gh_file;
            current_move[2] = (knights >> 6) & not_gh_file;
            current_move[3] = (knights >> 15) & not_h_file;
            current_move[7] = (knights << 15) & not_a_file;
            current_move[6] = (knights << 6) & not_ab_file;
            current_move[5] = (knights >> 10) & not_ab_file;
            current_move[4] = (knights >> 17) & not_a_file;
            moves[i] = current_move.iter().fold(0, |x, y| x | y);
        }
        print!("[");
        let mut first = true;
        for i in moves {
            if first == true {
                first = false;
            } else {
                print!(", ")
            }
            print!("0x{:016X}u64", i);
        }
        print!("]");
    }
    #[test]
    fn generate_king_moves() {
        let not_a_file = 0x7F7F7F7F7F7F7F7Fu64;
        let not_h_file = 0xFEFEFEFEFEFEFEFEu64;
        let mut moves = [0u64; 64];
        for i in 0..64 {
            let king = 1 << i;
            moves[i] = ((king << 9 | king << 1 | king >> 7) & not_h_file)
                | king << 8
                | ((king << 7 | king >> 1 | king >> 9) & not_a_file)
                | king >> 8;
        }
        print!("[");
        let mut first = true;
        for i in moves {
            if first == true {
                first = false;
            } else {
                print!(", ")
            }
            print!("0x{:016X}u64", i);
        }
        print!("]");
    }
    #[test]
    fn generate_rook_mask() {
        let mut mask = [0u64; 64];
        let mut file_mask = [0u64; 8];
        let mut rank_mask = [0u64; 8];
        for i in 0..8 {
            file_mask[i] = 0x0101010101010101u64 << i;
            rank_mask[i] = 0xFFu64 << i * 8;
        }
        print!("[");
        let mut first = true;
        for i in 0..64 {
            let mut bitmask = 0u64;
            if i / 8 != 7 {
                bitmask |= 0xFF00000000000000u64
            };
            if i / 8 != 0 {
                bitmask |= 0xFFu64
            };
            if i % 8 != 7 {
                bitmask |= 0x8080808080808080u64
            };
            if i % 8 != 0 {
                bitmask |= 0x0101010101010101u64
            };
            mask[i] = (file_mask[i % 8] | rank_mask[i / 8]) & !(1 << i); //& !bitmask;
            if first == true {
                first = false;
            } else {
                print!(", ")
            }
            print!("0x{:016X}u64", mask[i]);
        }
        println!("]");
    }
    #[test]
    fn generate_bishop_mask() {
        let main_diag = 0x8040201008040201u64;
        let antimain_diag = 0x0102040810204080u64;
        let mut mask = [0u64; 64];

        print!("[");
        let mut first = true;
        for i in 0..64isize {
            let diag = 8 * (i & 7) - (i & 56);
            let diag_north = -diag & (diag >> 31);
            let diag_south = diag & (-diag >> 31);
            let anti_diag = 56 - 8 * (i & 7) - (i & 56);
            let anti_diag_north = -anti_diag & (anti_diag >> 31);
            let anti_diag_south = anti_diag & (-anti_diag >> 31);
            mask[i as usize] = (((main_diag as u64 >> diag_south) << diag_north)
                | ((antimain_diag as u64 >> anti_diag_south) << anti_diag_north))
                & !(1 << i); //& !(0xFF818181818181FFu64);

            if first == true {
                first = false;
            } else {
                print!(", ");
            }
            print!("0x{:016X}u64", mask[i as usize]);
        }
        println!("]");
    }

#[derive(PartialEq, Eq)]
    enum Piece {Rook, Bishop}

    impl Piece {
        fn moves(&self, square: u64, blockers: u64) -> u64{
            let up_blocked = blockers | 0xFF00000000000000u64;
            let down_blocked = blockers | 0xFFu64;
            let west_blocked = blockers | 0x8080808080808080u64;
            let east_blocked = blockers | 0x0101010101010101u64;
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
                    up_dir += 1;
                    valid_moves |= piece_square << (8 * up_dir);
                    if (piece_square << (8 * up_dir)) & !up_blocked == 0 {
                        break 'up;
                    }
                }
                'down: loop {
                    down_dir += 1;
                    valid_moves |= piece_square >> (8 * down_dir);
                    if (piece_square >> (8 * down_dir)) & !down_blocked == 0 {
                        break 'down;
                    }
                }
                'west: loop {
                    west_dir += 1;
                    valid_moves |= piece_square << west_dir;
                    if (piece_square << west_dir) & !west_blocked == 0 {
                        break 'west;
                    }
                }
                'east: loop {
                    east_dir += 1;
                    valid_moves |= piece_square >> east_dir;
                    if (piece_square >> east_dir) & !east_blocked == 0 {
                        break 'east;
                    }
                }
            }
            else {
                let mut up_east_dir = 0;
                let mut up_west_dir = 0;
                let mut down_east_dir = 0;
                let mut down_west_dir = 0;
                'up_east: loop {
                    up_east_dir += 1;
                    valid_moves |= piece_square << (7 * up_east_dir);
                    if (piece_square << (7 * up_east_dir)) & !up_east_blocked == 0 {
                        break 'up_east;
                    }
                }
                'up_west: loop {
                    up_west_dir += 1;
                    valid_moves |= piece_square << (9 * up_west_dir);
                    if (piece_square << (9 * up_west_dir)) & !up_west_blocked == 0 {
                        break 'up_west;
                    }
                }
                'down_west: loop {
                    down_west_dir += 1;
                    valid_moves |= piece_square >> (7 * down_west_dir);
                    if (piece_square >> (7* down_west_dir)) & !down_west_blocked == 0 {
                        break 'down_west;
                    }
                }
                'down_east: loop {
                    down_east_dir += 1;
                    valid_moves |= piece_square >> (9 *down_east_dir);
                    if (piece_square >> (9 * down_east_dir)) & !down_east_blocked == 0 {
                        break 'down_east;
                    }
                }
            }
            valid_moves
        }
    }

    #[test]
    fn generate_rook_magics() {
        let mut array: Vec<(u64, Vec<u64>)> = vec![(0, vec![]);64];
        let bits = 12;
        for square in 0..64 {
            't: loop {
                let magic = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
                let rook_mask = crate::precomputed::rook_mask[square as usize];
                if let Ok(table) = generate_table(magic, rook_mask, bits, square, Piece::Rook) {
                    println!("index: {:?}", square);

                    array[square as usize] = (magic, table);
                    //println!("(magic: {:?}, bits: {:?}, table: {:?})", magic, bits, table);
                    break 't;
                }
            }
        }
        let mut string = format!("static rook_magics : [u64;64] = [");
        let mut string2 = format!("static rook_attacks: [[u64;4096];64] = [");
        let mut first = true;
        for (magic, list) in array {
            if first == true {
                first = false;
            }
            else {
                string.push_str(", ");
                string2.push_str(", ");
            }
            string.push_str(&mut format!("0x{:016X}", magic));
            string2.push_str("[");
            let mut first_list = true;
            for i in list {
                if first_list == true {
                    first_list = false;
                }
                else {
                    string2.push_str(", ");
                }
                string2.push_str(&mut format!("0x{:016X}", i));
            }
            string2.push_str("]");
        };
        string.push_str("];");
        string2.push_str("];");
        let mut file = std::fs::File::create("rook_magic.rs").unwrap();
        let _ = file.write_all(string.as_bytes());
        let _ = file.write_all(string2.as_bytes());
    }

    #[test]
    fn generate_bishop_magics() {
        let mut array: Vec<(u64, Vec<u64>)> = vec![(0, vec![]);64];
        let bits = 9;
        for square in 0..64 {
            't: loop {
                let magic = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
                let bishop_mask = crate::precomputed::bishop_mask[square as usize];
                if let Ok(table) = generate_table(magic, bishop_mask, bits, square, Piece::Bishop) {
                    println!("index: {:?}", square);

                    array[square as usize] = (magic, table);
                    //println!("(magic: {:?}, bits: {:?}, table: {:?})", magic, bits, table);
                    break 't;
                }
            }
        }
        let mut string = format!("static rook_magics : [u64;64] = [");
        let mut string2 = format!("static rook_attacks: [[u64;512];64] = [");
        let mut first = true;
        for (magic, list) in array {
            if first == true {
                first = false;
            }
            else {
                string.push_str(", ");
                string2.push_str(", ");
            }
            string.push_str(&mut format!("0x{:016X}", magic));
            string2.push_str("[");
            let mut first_list = true;
            for i in list {
                if first_list == true {
                    first_list = false;
                }
                else {
                    string2.push_str(", ");
                }
                string2.push_str(&mut format!("0x{:016X}", i));
            }
            string2.push_str("]");
        };
        string.push_str("];");
        string2.push_str("];");
        let mut file = std::fs::File::create("bishop_magic.rs").unwrap();
        let _ = file.write_all(string.as_bytes());
        let _ = file.write_all(string2.as_bytes());
    }

    fn generate_table(magic: u64, piece_mask: u64, bits: u64, square: u64, piece: Piece) -> Result<Vec<u64>, ()> {
        let mut table = vec![0u64; 1 << bits];
        for permutation in permutations(piece_mask) {
            let moves = piece.moves(square, permutation);
            let entry = &mut table[crate::precomputed::magic_to_index(magic, permutation, bits)];
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
        for i in 0u64..permutation_count {
            for j in 0u64..bits.len() as u64 {
                let bit = (i >> j) & 1;
                permutations[i as usize] |= bit << bits[j as usize];
            }
        }
        permutations
    }
}


fn magic_to_index(magic: u64, permutation: u64, bits: u64) -> usize {
    (permutation.wrapping_mul(magic) >> (64 - bits)) as usize
}

pub static knight_moves: [u64; 64] = [
    0x0000000000020400u64,
    0x0000000000050800u64,
    0x00000000000A1100u64,
    0x0000000000142200u64,
    0x0000000000284400u64,
    0x0000000000508800u64,
    0x0000000000A01000u64,
    0x0000000000402000u64,
    0x0000000002040004u64,
    0x0000000005080008u64,
    0x000000000A110011u64,
    0x0000000014220022u64,
    0x0000000028440044u64,
    0x0000000050880088u64,
    0x00000000A0100010u64,
    0x0000000040200020u64,
    0x0000000204000402u64,
    0x0000000508000805u64,
    0x0000000A1100110Au64,
    0x0000001422002214u64,
    0x0000002844004428u64,
    0x0000005088008850u64,
    0x000000A0100010A0u64,
    0x0000004020002040u64,
    0x0000020400040200u64,
    0x0000050800080500u64,
    0x00000A1100110A00u64,
    0x0000142200221400u64,
    0x0000284400442800u64,
    0x0000508800885000u64,
    0x0000A0100010A000u64,
    0x0000402000204000u64,
    0x0002040004020000u64,
    0x0005080008050000u64,
    0x000A1100110A0000u64,
    0x0014220022140000u64,
    0x0028440044280000u64,
    0x0050880088500000u64,
    0x00A0100010A00000u64,
    0x0040200020400000u64,
    0x0204000402000000u64,
    0x0508000805000000u64,
    0x0A1100110A000000u64,
    0x1422002214000000u64,
    0x2844004428000000u64,
    0x5088008850000000u64,
    0xA0100010A0000000u64,
    0x4020002040000000u64,
    0x0400040200000000u64,
    0x0800080500000000u64,
    0x1100110A00000000u64,
    0x2200221400000000u64,
    0x4400442800000000u64,
    0x8800885000000000u64,
    0x100010A000000000u64,
    0x2000204000000000u64,
    0x0004020000000000u64,
    0x0008050000000000u64,
    0x00110A0000000000u64,
    0x0022140000000000u64,
    0x0044280000000000u64,
    0x0088500000000000u64,
    0x0010A00000000000u64,
    0x0020400000000000u64,
];
pub static king_moves: [u64; 64] = [
    0x0000000000000302u64,
    0x0000000000000705u64,
    0x0000000000000E0Au64,
    0x0000000000001C14u64,
    0x0000000000003828u64,
    0x0000000000007050u64,
    0x000000000000E0A0u64,
    0x000000000000C040u64,
    0x0000000000030203u64,
    0x0000000000070507u64,
    0x00000000000E0A0Eu64,
    0x00000000001C141Cu64,
    0x0000000000382838u64,
    0x0000000000705070u64,
    0x0000000000E0A0E0u64,
    0x0000000000C040C0u64,
    0x0000000003020300u64,
    0x0000000007050700u64,
    0x000000000E0A0E00u64,
    0x000000001C141C00u64,
    0x0000000038283800u64,
    0x0000000070507000u64,
    0x00000000E0A0E000u64,
    0x00000000C040C000u64,
    0x0000000302030000u64,
    0x0000000705070000u64,
    0x0000000E0A0E0000u64,
    0x0000001C141C0000u64,
    0x0000003828380000u64,
    0x0000007050700000u64,
    0x000000E0A0E00000u64,
    0x000000C040C00000u64,
    0x0000030203000000u64,
    0x0000070507000000u64,
    0x00000E0A0E000000u64,
    0x00001C141C000000u64,
    0x0000382838000000u64,
    0x0000705070000000u64,
    0x0000E0A0E0000000u64,
    0x0000C040C0000000u64,
    0x0003020300000000u64,
    0x0007050700000000u64,
    0x000E0A0E00000000u64,
    0x001C141C00000000u64,
    0x0038283800000000u64,
    0x0070507000000000u64,
    0x00E0A0E000000000u64,
    0x00C040C000000000u64,
    0x0302030000000000u64,
    0x0705070000000000u64,
    0x0E0A0E0000000000u64,
    0x1C141C0000000000u64,
    0x3828380000000000u64,
    0x7050700000000000u64,
    0xE0A0E00000000000u64,
    0xC040C00000000000u64,
    0x0203000000000000u64,
    0x0507000000000000u64,
    0x0A0E000000000000u64,
    0x141C000000000000u64,
    0x2838000000000000u64,
    0x5070000000000000u64,
    0xA0E0000000000000u64,
    0x40C0000000000000u64,
];

static rook_mask: [u64; 64] = [
    0x000101010101017Eu64,
    0x000202020202027Cu64,
    0x000404040404047Au64,
    0x0008080808080876u64,
    0x001010101010106Eu64,
    0x002020202020205Eu64,
    0x004040404040403Eu64,
    0x008080808080807Eu64,
    0x0001010101017E00u64,
    0x0002020202027C00u64,
    0x0004040404047A00u64,
    0x0008080808087600u64,
    0x0010101010106E00u64,
    0x0020202020205E00u64,
    0x0040404040403E00u64,
    0x0080808080807E00u64,
    0x00010101017E0100u64,
    0x00020202027C0200u64,
    0x00040404047A0400u64,
    0x0008080808760800u64,
    0x00101010106E1000u64,
    0x00202020205E2000u64,
    0x00404040403E4000u64,
    0x00808080807E8000u64,
    0x000101017E010100u64,
    0x000202027C020200u64,
    0x000404047A040400u64,
    0x0008080876080800u64,
    0x001010106E101000u64,
    0x002020205E202000u64,
    0x004040403E404000u64,
    0x008080807E808000u64,
    0x0001017E01010100u64,
    0x0002027C02020200u64,
    0x0004047A04040400u64,
    0x0008087608080800u64,
    0x0010106E10101000u64,
    0x0020205E20202000u64,
    0x0040403E40404000u64,
    0x0080807E80808000u64,
    0x00017E0101010100u64,
    0x00027C0202020200u64,
    0x00047A0404040400u64,
    0x0008760808080800u64,
    0x00106E1010101000u64,
    0x00205E2020202000u64,
    0x00403E4040404000u64,
    0x00807E8080808000u64,
    0x007E010101010100u64,
    0x007C020202020200u64,
    0x007A040404040400u64,
    0x0076080808080800u64,
    0x006E101010101000u64,
    0x005E202020202000u64,
    0x003E404040404000u64,
    0x007E808080808000u64,
    0x7E01010101010100u64,
    0x7C02020202020200u64,
    0x7A04040404040400u64,
    0x7608080808080800u64,
    0x6E10101010101000u64,
    0x5E20202020202000u64,
    0x3E40404040404000u64,
    0x7E80808080808000u64,
];

static bishop_mask: [u64; 64] = [
    0x0040201008040200u64,
    0x0000402010080400u64,
    0x0000004020100A00u64,
    0x0000000040221400u64,
    0x0000000002442800u64,
    0x0000000204085000u64,
    0x0000020408102000u64,
    0x0002040810204000u64,
    0x0020100804020000u64,
    0x0040201008040000u64,
    0x00004020100A0000u64,
    0x0000004022140000u64,
    0x0000000244280000u64,
    0x0000020408500000u64,
    0x0002040810200000u64,
    0x0004081020400000u64,
    0x0010080402000200u64,
    0x0020100804000400u64,
    0x004020100A000A00u64,
    0x0000402214001400u64,
    0x0000024428002800u64,
    0x0002040850005000u64,
    0x0004081020002000u64,
    0x0008102040004000u64,
    0x0008040200020400u64,
    0x0010080400040800u64,
    0x0020100A000A1000u64,
    0x0040221400142200u64,
    0x0002442800284400u64,
    0x0004085000500800u64,
    0x0008102000201000u64,
    0x0010204000402000u64,
    0x0004020002040800u64,
    0x0008040004081000u64,
    0x00100A000A102000u64,
    0x0022140014224000u64,
    0x0044280028440200u64,
    0x0008500050080400u64,
    0x0010200020100800u64,
    0x0020400040201000u64,
    0x0002000204081000u64,
    0x0004000408102000u64,
    0x000A000A10204000u64,
    0x0014001422400000u64,
    0x0028002844020000u64,
    0x0050005008040200u64,
    0x0020002010080400u64,
    0x0040004020100800u64,
    0x0000020408102000u64,
    0x0000040810204000u64,
    0x00000A1020400000u64,
    0x0000142240000000u64,
    0x0000284402000000u64,
    0x0000500804020000u64,
    0x0000201008040200u64,
    0x0000402010080400u64,
    0x0002040810204000u64,
    0x0004081020400000u64,
    0x000A102040000000u64,
    0x0014224000000000u64,
    0x0028440200000000u64,
    0x0050080402000000u64,
    0x0020100804020000u64,
    0x0040201008040200u64,
];