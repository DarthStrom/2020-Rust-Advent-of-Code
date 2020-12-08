pub fn solve1(lines: &Vec<&str>) -> usize {
    let program = get_program(lines);

    run(&program).err().unwrap()
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let program = get_program(lines);
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            let mut parts = line.split(' ');
            let op = parts.next().unwrap();
            match op {
                "nop" | "jmp" => Some(line_num),
                _ => None,
            }
        })
        .map(|line_num| {
            let mut hacked_program = program.clone();
            match program[line_num] {
                ("jmp", amt) => hacked_program[line_num] = ("nop", amt),
                ("nop", amt) => hacked_program[line_num] = ("jmp", amt),
                _ => (),
            }
            hacked_program
        })
        .map(|program| run(&program))
        .filter(|result| result.is_ok())
        .take(1)
        .map(|result| result.unwrap())
        .collect::<Vec<usize>>()[0]
}

fn get_program<'a>(lines: &'a Vec<&str>) -> Vec<(&'a str, i32)> {
    let mut program = vec![];

    for line in lines {
        let mut parts = line.split(' ');
        let op = parts.next().unwrap();
        let amt = parts.next().unwrap().parse::<i32>().unwrap();
        program.push((op, amt));
    }

    program
}

fn run(program: &Vec<(&str, i32)>) -> Result<usize, usize> {
    let mut acc: usize = 0;
    let mut adr = 0;
    let mut seen = vec![];

    while !seen.contains(&adr) {
        if adr >= program.len() {
            return Ok(acc);
        }
        seen.push(adr);
        match program[adr] {
            ("acc", amt) => {
                acc = (acc as i32 + amt) as usize;
                adr += 1;
            }
            ("jmp", amt) => adr = (adr as i32 + amt) as usize,
            ("nop", _) => adr += 1,
            _ => panic!("unknown instruction"),
        }
    }

    Err(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn example1() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 5);
    }

    #[test]
    fn example2() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 8);
    }
}
