pub mod rook_magic;
pub mod bishop_magic;
pub mod precomputed;

pub fn magic_to_index(magic: u64, permutation: u64, bits: u64) -> usize {
    (permutation.wrapping_mul(magic) >> (64 - bits)) as usize
}