use crate::puzzle::Puzzle;

pub struct Puzzle1 {}

impl Puzzle1 {
    fn calculate_fuel(&self, mass: i32) -> i32 {
        (mass / 3) - 2
    }
    fn calculate_fuel_recursive(&self, mass: i32) -> i32 {
        let fuel = self.calculate_fuel(mass);
        if fuel > 0 {
            return fuel + self.calculate_fuel_recursive(fuel);
        }
        return 0;
    }
}

impl Puzzle<Vec<i32>> for Puzzle1 {
    fn parse_input(&self, input: &str) -> Vec<i32> {
        input
            .split("\r\n")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        return input.into_iter().map(|x| self.calculate_fuel(*x)).sum();
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        return input
            .into_iter()
            .map(|x| self.calculate_fuel_recursive(*x))
            .sum();
    }
}
