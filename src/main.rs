use std::env::args;
use std::fs::read_to_string;

mod day1;
mod day2;

fn main() {
    let mut args = args();
    let _executable = args.next().unwrap();
    if let Some(arg) = args.next() {
        if let Ok(day) = arg.to_lowercase().trim_start_matches("day").parse::<u32>() {
            solve(day);
        } else {
            eprintln!("could not parse day: \"{}\"", arg);
        }
    } else {
        eprintln!("which day?");
    }
}

fn solve(day: u32) {
    if let Ok(input) = read_to_string(format!("./inputs/day{}.txt", day)) {
        let lines = input.lines().collect::<Vec<_>>();

        match day {
            1 => {
                println!("Day1, Part1: {}", day1::solve1(&lines));
                println!("Day2, Part2: {}", day1::solve2(&lines));
            }
            2 => {
                println!("Day2, Part1: {}", day2::solve1(&lines));
                println!("Day2, Part2: {}", day2::solve2(&lines));
            }
            _ => unimplemented!(),
        }
    } else {
        eprintln!("could not find input file");
    }
}
