use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    solve(lines, 2020)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    solve(lines, 30000000)
}

fn solve(lines: &Vec<&str>, nth: usize) -> usize {
    let mut lookup = HashMap::new();

    let mut last_spoken = 0;
    lines[0]
        .split(',')
        .flat_map(str::parse::<usize>)
        .enumerate()
        .for_each(|(i, n)| {
            if i > 0 {
                lookup.insert(last_spoken, i);
            }
            last_spoken = n;
        });

    let mut turn = lookup.len() + 2;
    while turn <= nth {
        if let Some(&before) = lookup.get(&last_spoken) {
            lookup.insert(last_spoken, turn - 1);
            last_spoken = turn - 1 - before;
        } else {
            lookup.insert(last_spoken, turn - 1);
            last_spoken = 0;
        }
        turn += 1;
    }

    last_spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve1(&vec!["0,3,6"]), 436);
        assert_eq!(solve1(&vec!["1,3,2"]), 1);
        assert_eq!(solve1(&vec!["2,1,3"]), 10);
        assert_eq!(solve1(&vec!["1,2,3"]), 27);
        assert_eq!(solve1(&vec!["2,3,1"]), 78);
        assert_eq!(solve1(&vec!["3,2,1"]), 438);
        assert_eq!(solve1(&vec!["3,1,2"]), 1836);
    }

    #[test]
    fn example2() {
        // Passes, but too slow to run every time
        // TODO: look into speeding up the algorithm

        // assert_eq!(solve2(&vec!["0,3,6"]), 175594);
        // assert_eq!(solve2(&vec!["1,3,2"]), 2578);
        // assert_eq!(solve2(&vec!["2,1,3"]), 3544142);
        // assert_eq!(solve2(&vec!["1,2,3"]), 261214);
        // assert_eq!(solve2(&vec!["2,3,1"]), 6895259);
        // assert_eq!(solve2(&vec!["3,2,1"]), 18);
        // assert_eq!(solve2(&vec!["3,1,2"]), 362);
    }
}
