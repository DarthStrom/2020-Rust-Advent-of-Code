use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    let mut error_rate = 0;
    let mut ranges = HashMap::new();
    lines
        .iter()
        .take_while(|&&l| !l.is_empty())
        .for_each(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap();
            let valid_nums = parts
                .next()
                .unwrap()
                .split(" or ")
                .flat_map(|range_str| {
                    let nums = range_str.trim().split("-").collect::<Vec<_>>();
                    let min = nums[0].parse::<usize>().unwrap();
                    let max = nums[1].parse::<usize>().unwrap();
                    (min..=max).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            ranges.insert(key, valid_nums);
        });

    lines
        .iter()
        .skip_while(|&&line| line != "nearby tickets:")
        .skip(1)
        .for_each(|line| {
            line.split(',').flat_map(str::parse::<usize>).for_each(|n| {
                if ranges.values().all(|v| !v.contains(&n)) {
                    error_rate += n;
                }
            });
        });

    error_rate
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 71);
    }
}
