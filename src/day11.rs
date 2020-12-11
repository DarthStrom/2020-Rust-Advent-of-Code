pub fn solve1(lines: &Vec<&str>) -> usize {
    let mut matrix = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut last_occupied = get_occupied(&matrix);
    matrix = iterate(&matrix);
    let mut occupied = get_occupied(&matrix);

    while occupied != last_occupied {
        matrix = iterate(&matrix);
        last_occupied = occupied;
        occupied = get_occupied(&matrix);
    }

    occupied
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    0
}

fn get_occupied(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter().fold(0, |acc, cur| {
        acc + cur.iter().fold(0, |acc, cur| acc + occupied(cur))
    })
}

fn occupied(c: &char) -> usize {
    if c == &'#' {
        1
    } else {
        0
    }
}

fn iterate(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix
        .iter()
        .enumerate()
        .map(|(j, y)| {
            y.iter()
                .enumerate()
                .map(|(i, x)| match x {
                    'L' => match adjacent_occupied(matrix, j, i) {
                        0 => '#',
                        _ => 'L',
                    },
                    '#' => match adjacent_occupied(matrix, j, i) {
                        n if n >= 4 => 'L',
                        _ => '#',
                    },
                    &state => state,
                })
                .collect()
        })
        .collect()
}

fn adjacent_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut result = 0;
    let max_row = matrix.len() - 1;
    let max_col = matrix[0].len() - 1;
    if row > 0 {
        if col > 0 {
            result += occupied(&matrix[row - 1][col - 1]);
        }
        result += occupied(&matrix[row - 1][col]);
        if col < max_col {
            result += occupied(&matrix[row - 1][col + 1]);
        }
    }
    if col > 0 {
        result += occupied(&matrix[row][col - 1]);
    }
    if col < max_col {
        result += occupied(&matrix[row][col + 1]);
    }
    if row < max_row {
        if col > 0 {
            result += occupied(&matrix[row + 1][col - 1]);
        }
        result += occupied(&matrix[row + 1][col]);
        if col < max_col {
            result += occupied(&matrix[row + 1][col + 1]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let seats = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .lines()
            .collect::<Vec<&str>>();

        assert_eq!(solve1(&seats), 37);
    }
}
