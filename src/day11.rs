pub fn solve1(lines: &Vec<&str>) -> usize {
    solve(lines, &iterate_adjacent)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    solve(lines, &iterate_vision)
}

fn solve(lines: &Vec<&str>, iteration: &dyn Fn(&Vec<Vec<char>>) -> Vec<Vec<char>>) -> usize {
    let mut matrix = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut last_occupied = get_occupied(&matrix);
    matrix = iteration(&matrix);
    let mut occupied = get_occupied(&matrix);

    while occupied != last_occupied {
        matrix = iteration(&matrix);
        last_occupied = occupied;
        occupied = get_occupied(&matrix);
    }

    occupied
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

fn iterate_adjacent(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn iterate_vision(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix
        .iter()
        .enumerate()
        .map(|(j, y)| {
            y.iter()
                .enumerate()
                .map(|(i, x)| match x {
                    'L' => match vision_occupied(matrix, j, i) {
                        0 => '#',
                        _ => 'L',
                    },
                    '#' => match vision_occupied(matrix, j, i) {
                        n if n >= 5 => 'L',
                        _ => '#',
                    },
                    &state => state,
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>()
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

fn vision_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut result = 0;
    result += nw_occupied(matrix, row, col);
    result += n_occupied(matrix, row, col);
    result += ne_occupied(matrix, row, col);
    result += e_occupied(matrix, row, col);
    result += se_occupied(matrix, row, col);
    result += s_occupied(matrix, row, col);
    result += sw_occupied(matrix, row, col);
    result += w_occupied(matrix, row, col);
    result
}

fn nw_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row <= 0 || col <= 0 {
        0
    } else {
        match matrix[row - 1][col - 1] {
            '#' => 1,
            'L' => 0,
            '.' => nw_occupied(matrix, row - 1, col - 1),
            _ => panic!("invalid char"),
        }
    }
}

fn n_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row <= 0 {
        0
    } else {
        match matrix[row - 1][col] {
            '#' => 1,
            'L' => 0,
            '.' => n_occupied(matrix, row - 1, col),
            _ => panic!("invalid char"),
        }
    }
}

fn ne_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row <= 0 || col >= matrix[0].len() - 1 {
        0
    } else {
        match matrix[row - 1][col + 1] {
            '#' => 1,
            'L' => 0,
            '.' => ne_occupied(matrix, row - 1, col + 1),
            _ => panic!("invalid char"),
        }
    }
}

fn e_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if col >= matrix[0].len() - 1 {
        0
    } else {
        match matrix[row][col + 1] {
            '#' => 1,
            'L' => 0,
            '.' => e_occupied(matrix, row, col + 1),
            _ => panic!("invalid char"),
        }
    }
}

fn se_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row >= matrix.len() - 1 || col >= matrix[0].len() - 1 {
        0
    } else {
        match matrix[row + 1][col + 1] {
            '#' => 1,
            'L' => 0,
            '.' => se_occupied(matrix, row + 1, col + 1),
            _ => panic!("invalid char"),
        }
    }
}

fn s_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row >= matrix.len() - 1 {
        0
    } else {
        match matrix[row + 1][col] {
            '#' => 1,
            'L' => 0,
            '.' => s_occupied(matrix, row + 1, col),
            _ => panic!("invalid char"),
        }
    }
}

fn sw_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row >= matrix.len() - 1 || col <= 0 {
        0
    } else {
        match matrix[row + 1][col - 1] {
            '#' => 1,
            'L' => 0,
            '.' => sw_occupied(matrix, row + 1, col - 1),
            _ => panic!("invalid char"),
        }
    }
}

fn w_occupied(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if col <= 0 {
        0
    } else {
        match matrix[row][col - 1] {
            '#' => 1,
            'L' => 0,
            '.' => w_occupied(matrix, row, col - 1),
            _ => panic!("invalid char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_LAYOUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn example1() {
        let seats = STARTING_LAYOUT.lines().collect::<Vec<&str>>();

        assert_eq!(solve1(&seats), 37);
    }

    #[test]
    fn example2() {
        let seats = STARTING_LAYOUT.lines().collect::<Vec<&str>>();

        assert_eq!(solve2(&seats), 26);
    }
}
