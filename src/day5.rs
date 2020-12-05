pub fn solve1(lines: &Vec<&str>) -> usize {
    lines.iter().map(get_seat_id).max().unwrap()
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let mut seat_ids = lines.iter().map(get_seat_id).collect::<Vec<_>>();
    seat_ids.sort();
    for i in 0..seat_ids.len() - 1 {
        if seat_ids[i + 1] - seat_ids[i] > 1 {
            return seat_ids[i] + 1;
        }
    }

    0
}

fn get_seat_id(line: &&str) -> usize {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;
    let chars = line.chars().collect::<Vec<_>>();

    for i in 0..7 {
        let half = (row_max - row_min + 1) / 2;
        match chars[i] {
            'F' => row_max -= half,
            'B' => row_min += half,
            _ => panic!("invalid character"),
        }
    }

    for i in 7..10 {
        let half = (col_max - col_min + 1) / 2;
        match chars[i] {
            'L' => col_max -= half,
            'R' => col_min += half,
            _ => panic!("invalid character"),
        }
    }

    row_min * 8 + col_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

        assert_eq!(solve1(&lines), 820)
    }
}
