use std::env::args;
use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

macro_rules! print_solution {
    ($m:ident, $n:expr, $l:expr) => {{
        println!("Day{}, Part1: {}", $n, $m::solve1($l));
        println!("Day{}, Part2: {}", $n, $m::solve2($l));
    }};
}

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
            1 => print_solution!(day1, day, &lines),
            2 => print_solution!(day2, day, &lines),
            3 => print_solution!(day3, day, &lines),
            4 => print_solution!(day4, day, &lines),
            5 => print_solution!(day5, day, &lines),
            6 => print_solution!(day6, day, &lines),
            7 => print_solution!(day7, day, &lines),
            8 => print_solution!(day8, day, &lines),
            _ => unimplemented!(),
        }
    } else {
        eprintln!("could not find input file");
    }
}
