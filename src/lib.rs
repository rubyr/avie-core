pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod evaluation;
mod gamestate;
pub mod parse;
pub mod board;
mod precomputed;

pub use evaluation::choose_best_move;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
