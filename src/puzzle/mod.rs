pub mod puzzle1;

pub trait Puzzle<TInput> {
    fn parse_input(&self, input: &str) -> TInput;
    fn part1(&self, input: &TInput) -> i32;
    fn part2(&self, input: &TInput) -> i32;
}