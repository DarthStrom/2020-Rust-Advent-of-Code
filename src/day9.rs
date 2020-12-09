pub fn solve1(lines: &Vec<&str>) -> usize {
    let nums = lines.iter().flat_map(|&s| s.parse::<usize>()).collect();

    solve(&nums, 25)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    0
}

fn solve(nums: &Vec<usize>, preamble_size: usize) -> usize {
    let rest = &nums[preamble_size..];

    for (i, &n) in rest.iter().enumerate() {
        let previous_five = &nums[i..i + preamble_size];
        if !sum_of_two_in(n, previous_five) {
            return n;
        }
    }

    0
}

fn sum_of_two_in(n: usize, nums: &[usize]) -> bool {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == n {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(solve(&nums, 5), 127);
    }
}
