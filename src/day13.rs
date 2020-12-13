pub fn solve1(lines: &Vec<&str>) -> usize {
    let earliest_timestamp = lines[0].parse::<usize>().unwrap();
    let (_, bus_id) = lines[1]
        .split(',')
        .flat_map(|s| s.parse::<usize>())
        .map(|mul| (lowest_above_earliest(mul, earliest_timestamp), mul))
        .min()
        .unwrap();
    bus_id * minutes_to_wait(bus_id, earliest_timestamp)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let busses = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, s)| (i, s.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    let mut time = 0;
    let mut jump = busses[0].1;
    for &(delta, bus) in busses[1..].iter() {
        if bus > 0 {
            while (time + delta) % bus != 0 {
                time += jump;
            }
            jump *= bus;
        }
    }

    time
}

fn lowest_above_earliest(mul: usize, earliest: usize) -> usize {
    let mut result = mul;
    while result < earliest {
        result += mul
    }
    result
}

fn minutes_to_wait(bus_id: usize, earliest: usize) -> usize {
    lowest_above_earliest(bus_id, earliest) - earliest
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn example1() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 295);
    }

    #[test]
    fn example2() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 1_068_781);
    }

    #[test]
    fn example3() {
        let lines = "\n17,x,13,19".lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 3_417);
    }

    #[test]
    fn example4() {
        let lines = "\n67,7,59,61".lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 754_018);
    }

    #[test]
    fn example5() {
        let lines = "\n67,x,7,59,61".lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 779_210);
    }

    #[test]
    fn example6() {
        let lines = "\n67,7,x,59,61".lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 1_261_476);
    }

    #[test]
    fn example7() {
        let lines = "\n1789,37,47,1889".lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 1_202_161_486);
    }
}
