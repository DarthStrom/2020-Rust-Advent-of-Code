pub fn solve1(lines: &Vec<&str>) -> usize {
    encountered_trees(lines, 3, 1)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let slope1 = encountered_trees(lines, 1, 1);
    let slope2 = encountered_trees(lines, 3, 1);
    let slope3 = encountered_trees(lines, 5, 1);
    let slope4 = encountered_trees(lines, 7, 1);
    let slope5 = encountered_trees(lines, 1, 2);

    slope1 * slope2 * slope3 * slope4 * slope5
}

fn encountered_trees(lines: &Vec<&str>, right: usize, down: usize) -> usize {
    let mut count = 0;
    let grid = lines
        .iter()
        .map(|&line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();
    for i in 0..height {
        let row = i * down;
        if row >= height {
            return count;
        }
        if grid[row][(i * right) % width] == '#' {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn example1() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 7);
    }

    #[test]
    fn example2() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 336);
    }
}
