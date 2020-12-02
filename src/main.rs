use std::fs::read_to_string;

mod day1;

fn main() {
    let input = read_to_string("./inputs/day1.txt").unwrap();
    let expenses = input
        .lines()
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();
    println!("Day1, Part1: {}", day1::solve1(&expenses));
    println!("Day1, Part2: {}", day1::solve2(&expenses));
}
