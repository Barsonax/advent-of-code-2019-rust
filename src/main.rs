mod puzzle;

use crate::puzzle::Puzzle;
use crate::puzzle::puzzle1;

fn main() {
    solve_puzzle(&puzzle1::Puzzle1 {});
}

fn solve_puzzle<T>(puzzle: &dyn Puzzle<T>){
    let input = include_str!("puzzle1.txt");
    let parsed_input = puzzle.parse_input(input);
    let result1 = puzzle.part1(&parsed_input);  
    println!("{}", result1);

    let result2 = puzzle.part2(&parsed_input);
    println!("{}", result2);
}
