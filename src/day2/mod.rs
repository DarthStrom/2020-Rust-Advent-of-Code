pub fn solve1(passwords: &Vec<&str>) -> usize {
    passwords
        .into_iter()
        .filter(|&&password| valid(password))
        .count()
}

pub fn solve2(passwords: &Vec<&str>) -> usize {
    passwords
        .into_iter()
        .filter(|&&password| really_valid(password))
        .count()
}

fn valid(password: &str) -> bool {
    let mut words = password.split(' ');
    let mut range_parts = words
        .next()
        .unwrap()
        .split('-')
        .flat_map(str::parse::<usize>);
    let min = range_parts.next().unwrap();
    let max = range_parts.next().unwrap();

    let letter = words.next().unwrap().chars().next().unwrap();

    let pass = words.next().unwrap();
    let count = pass.chars().filter(|&c| c == letter).count();
    count >= min && count <= max
}

fn really_valid(password: &str) -> bool {
    let mut words = password.split(' ');
    let mut range_parts = words
        .next()
        .unwrap()
        .split('-')
        .flat_map(str::parse::<usize>);
    let pos1 = range_parts.next().unwrap() - 1;
    let pos2 = range_parts.next().unwrap() - 1;

    let letter = words.next().unwrap().chars().next().unwrap();

    let pass = words.next().unwrap().chars().collect::<Vec<_>>();

    let letter1 = pass[pos1];
    let letter2 = pass[pos2];

    (letter1 == letter && letter2 != letter) || (letter2 == letter && letter1 != letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(solve1(&passwords), 2);
    }

    #[test]
    fn example2() {
        let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(solve2(&passwords), 1);
    }
}
