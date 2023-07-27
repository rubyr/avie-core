pub mod rook_magic;
pub mod bishop_magic;
pub mod moves;
fn _u64_to_board(board: u64) {
    for i in 0..8 {
        let row = (board >> (56 - (i * 8))) as u8;
        println!("{:08b}", row);
    }
    println!()
}

pub fn magic_to_index<const BITS: u64>(magic: u64, permutation: u64) -> usize {
    (permutation.wrapping_mul(magic) >> (64 - BITS)) as usize
}