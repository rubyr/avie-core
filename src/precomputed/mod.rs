pub mod rook_magic;
pub mod bishop_magic;
pub mod moves;
pub mod square_tables;
pub mod zobrist;

pub fn magic_to_index<const BITS: u64>(magic: u64, permutation: u64) -> usize {
    (permutation.wrapping_mul(magic) >> (64 - BITS)) as usize
}