pub fn num_valid(passwords: &Vec<&str>) -> usize {
    passwords
        .into_iter()
        .filter(|&&password| valid(password))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(num_valid(&passwords), 2);
    }
}
