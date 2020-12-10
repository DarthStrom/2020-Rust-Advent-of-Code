pub fn solve1(lines: &Vec<&str>) -> usize {
    let groups = lines.split(|line| line.is_empty());
    groups
        .map(|group| {
            let mut group_vec = group.to_vec().join("").chars().collect::<Vec<_>>();
            group_vec.sort();
            group_vec.dedup();
            group_vec.len()
        })
        .sum()
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let groups = lines.split(|line| line.is_empty());
    groups
        .map(|group| {
            let mut shared = group[0].chars().collect::<Vec<_>>();
            for i in 1..group.len() {
                for &c in shared.clone().iter() {
                    if !group[i].chars().collect::<Vec<_>>().contains(&c) {
                        let index = shared.iter().position(|x| *x == c).unwrap();
                        shared.remove(index);
                    }
                }
            }
            shared.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 11);
    }

    #[test]
    fn example2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 6);
    }
}
