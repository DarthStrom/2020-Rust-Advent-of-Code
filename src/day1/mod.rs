pub fn solve1(expenses: &Vec<u32>) -> u32 {
    let len = expenses.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            let a = expenses[i];
            let b = expenses[j];
            if expenses[i] + expenses[j] == 2020 {
                return a * b;
            }
        }
    }
    0
}

pub fn solve2(expenses: &Vec<u32>) -> u32 {
    let len = expenses.len();
    for i in 0..len - 2 {
        for j in i + 1..len - 1 {
            for k in j + 1..len {
                let a = expenses[i];
                let b = expenses[j];
                let c = expenses[k];
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve1(&expenses), 514_579);
    }

    #[test]
    fn example2() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve2(&expenses), 241_861_950);
    }
}
