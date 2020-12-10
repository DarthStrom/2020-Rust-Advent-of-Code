use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    let adapters = lines.iter().flat_map(|l| l.parse::<usize>()).collect();

    solve(&adapters)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let adapters = lines.iter().flat_map(|l| l.parse::<usize>()).collect();

    arrangements(&adapters)
}

fn solve(adapters: &Vec<usize>) -> usize {
    let mut sorted = adapters.clone();
    sorted.sort();

    let mut ones = 0;
    let mut threes = 0;
    let mut last = 0;

    for adapter in sorted {
        match adapter - last {
            1 => ones += 1,
            2 => (),
            3 => threes += 1,
            _ => panic!("chain broken!"),
        }
        last = adapter;
    }

    ones * (threes + 1)
}

fn arrangements(adapters: &Vec<usize>) -> usize {
    let mut sorted = adapters.clone();
    sorted.sort();
    sorted.insert(0, 0);

    total_arrangements(&sorted, 0, &mut HashMap::new())
}

fn total_arrangements(
    sorted_adapters: &Vec<usize>,
    i: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if i == sorted_adapters.len() - 1 {
        return 1;
    }
    let start = sorted_adapters[i];
    let total = sorted_adapters
        .iter()
        .enumerate()
        .skip(i + 1)
        .take_while(|(_, x)| (1..=3).contains(&(*x - start)))
        .map(|(i, _)| {
            memo.get(&i)
                .copied()
                .unwrap_or_else(|| total_arrangements(&sorted_adapters, i, memo))
        })
        .sum::<usize>();
    memo.insert(i, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADAPTER_SET_1: [usize; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const ADAPTER_SET_2: [usize; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn example1() {
        assert_eq!(solve(&ADAPTER_SET_1.to_vec()), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(solve(&ADAPTER_SET_2.to_vec()), 220);
    }

    #[test]
    fn example3() {
        assert_eq!(arrangements(&ADAPTER_SET_1.to_vec()), 8);
    }

    #[test]
    fn example4() {
        assert_eq!(arrangements(&ADAPTER_SET_2.to_vec()), 19208);
    }
}
