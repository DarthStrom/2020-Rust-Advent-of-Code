use std::collections::HashMap;

use regex::Regex;

pub fn solve1(lines: &Vec<&str>) -> usize {
    solve(lines, &write_mem_masked)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    solve(lines, &write_mem_decoded)
}

fn solve(
    lines: &Vec<&str>,
    write_mem: &dyn Fn(&mut HashMap<usize, usize>, &str, &str, &str),
) -> usize {
    let mut memory = HashMap::new();
    let mut m = "";

    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "mask" => m = parts[2],
            mem_inst => write_mem(&mut memory, mem_inst, parts[2], m),
        }
    }

    memory.values().sum()
}

fn decode(mask: &str) -> (Vec<usize>, usize) {
    let enable = mask.chars().fold(0, |acc, x| {
        (acc << 1)
            | match x {
                '1' => 1,
                _ => 0,
            }
    });
    let disable = mask.chars().fold(0, |acc, x| {
        (acc << 1)
            | match x {
                'X' => 0,
                _ => 1,
            }
    });
    let float = mask
        .chars()
        .rev()
        .enumerate()
        .filter(|&(_, x)| x == 'X')
        .map(|(n, _)| 1 << n)
        .collect::<Vec<_>>();
    (
        (0..1 << float.len())
            .map(|x| {
                ((0..float.len())
                    .filter(|y| x & (1 << y) != 0)
                    .map(|y| float[y])
                    .sum::<usize>())
                    | enable
            })
            .collect::<Vec<_>>(),
        disable,
    )
}

fn mask(m: &str, value: usize) -> usize {
    let or = m.chars().fold(0, |acc, c| {
        (acc << 1)
            | match c {
                '1' => 1,
                _ => 0,
            }
    });
    let and = m.chars().fold(0, |acc, c| {
        (acc << 1)
            | match c {
                '0' => 0,
                _ => 1,
            }
    });
    (value & and) | or
}

fn write_mem_decoded(memory: &mut HashMap<usize, usize>, mem_inst: &str, value: &str, m: &str) {
    let re = Regex::new(r"mem\[(\d+)\]").unwrap();
    let encoded_address = re
        .captures_iter(mem_inst)
        .take(1)
        .flat_map(|c| c[1].parse::<usize>())
        .collect::<Vec<_>>()[0];

    let (enables, disable) = decode(m);
    for enable in enables {
        let address = encoded_address & disable | enable;
        memory.insert(address as usize, value.parse::<usize>().unwrap());
    }
}

fn write_mem_masked(memory: &mut HashMap<usize, usize>, mem_inst: &str, value: &str, m: &str) {
    let re = Regex::new(r"mem\[(\d+)\]").unwrap();
    let address = re
        .captures_iter(mem_inst)
        .take(1)
        .flat_map(|c| c[1].parse::<usize>())
        .collect::<Vec<_>>()[0];

    memory.insert(address, mask(m, value.parse::<usize>().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 165);
    }

    #[test]
    fn example2() {
        let lines = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 208);
    }
}
