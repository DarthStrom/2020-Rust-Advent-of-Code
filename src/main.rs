use std::env::args;
use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
    if let Ok(input) = read_to_string(format!("./inputs/day{:02}.txt", day)) {
        let lines = input.lines().collect::<Vec<_>>();

        match day {
            1 => print_solution!(day01, day, &lines),
            2 => print_solution!(day02, day, &lines),
            3 => print_solution!(day03, day, &lines),
            4 => print_solution!(day04, day, &lines),
            5 => print_solution!(day05, day, &lines),
            6 => print_solution!(day06, day, &lines),
            7 => print_solution!(day07, day, &lines),
            8 => print_solution!(day08, day, &lines),
            9 => print_solution!(day09, day, &lines),
            10 => print_solution!(day10, day, &lines),
            11 => print_solution!(day11, day, &lines),
            12 => print_solution!(day12, day, &lines),
            13 => print_solution!(day13, day, &lines),
            14 => print_solution!(day14, day, &lines),
            _ => unimplemented!(),
        }
    } else {
        eprintln!("could not find input file");
    }
}
