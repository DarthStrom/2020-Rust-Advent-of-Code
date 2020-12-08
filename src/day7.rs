use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    let mut rules = HashMap::new();

    for line in lines {
        let parts = line.split(" bags contain ").collect::<Vec<_>>();
        let color = parts[0];
        let contents = if parts[1] == "no other bags." {
            vec![]
        } else {
            parts[1]
                .trim_end_matches('.')
                .split(", ")
                .map(|s| s.trim_end_matches(" bags"))
                .map(|s| s.trim_end_matches(" bag"))
                .map(|s| s.trim_start_matches(|c: char| c.is_digit(10)))
                .map(|s| s.trim_start())
                .collect::<Vec<_>>()
        };

        rules.insert(color, contents);
    }

    rules
        .iter()
        .filter(|(k, _)| holds_bag(&rules, k, "shiny gold"))
        .count()
        - 1
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let mut rules = HashMap::new();

    for line in lines {
        let parts = line.split(" bags contain ").collect::<Vec<_>>();
        let color = parts[0];
        let contents = if parts[1] == "no other bags." {
            vec![]
        } else {
            parts[1]
                .trim_end_matches('.')
                .split(", ")
                .map(|s| s.trim_end_matches(" bags"))
                .map(|s| s.trim_end_matches(" bag"))
                .map(|s| {
                    let count = s[..1].parse::<usize>().unwrap();
                    (count, &s[2..])
                })
                .collect::<Vec<_>>()
        };

        rules.insert(color, contents);
    }

    rules["shiny gold"]
        .iter()
        .map(|(count, color)| count + count * bag_count(&rules, color))
        .sum()
}

fn bag_count(rules: &HashMap<&str, Vec<(usize, &str)>>, color: &str) -> usize {
    rules[color]
        .iter()
        .map(|(count, c)| count + count * bag_count(&rules, c))
        .sum()
}

fn holds_bag(rules: &HashMap<&str, Vec<&str>>, k: &str, bag: &str) -> bool {
    if k == bag {
        return true;
    } else {
        let sub_bags = &rules[k];
        sub_bags
            .iter()
            .any(|sub_bag| holds_bag(rules, sub_bag, bag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 4);
    }

    #[test]
    fn example2() {
        let lines = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 126);
    }
}
