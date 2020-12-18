use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn solve1(lines: &Vec<&str>) -> usize {
    let mut pocket = HashMap::new();
    let (mut min_x, mut max_x) = (-1, 1);
    let (mut min_y, mut max_y) = (-1, 1);
    let (mut min_z, mut max_z) = (-1, 1);

    lines.iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            min_x = min(x as i32 - 1, min_x);
            min_y = min(y as i32 - 1, min_y);
            max_x = max(x as i32 + 1, max_x);
            max_y = max(y as i32 + 1, max_y);

            pocket.insert((x as i32, y as i32, 0i32), c == '#');
        })
    });

    for _ in 0..6 {
        let last_pocket = pocket.clone();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if *last_pocket.get(&(x, y, z)).unwrap_or(&false) {
                        match active_3d_neighbors(&last_pocket, x, y, z) {
                            2 | 3 => (),
                            _ => {
                                pocket.insert((x, y, z), false);
                            }
                        }
                    } else {
                        match active_3d_neighbors(&last_pocket, x, y, z) {
                            3 => {
                                min_x = min(x - 1, min_x);
                                min_y = min(y - 1, min_y);
                                min_z = min(z - 1, min_z);
                                max_x = max(x + 1, max_x);
                                max_y = max(y + 1, max_y);
                                max_z = max(z + 1, max_z);

                                pocket.insert((x, y, z), true);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    pocket.values().filter(|&&v| v).count()
}

fn active_3d_neighbors(pocket: &HashMap<(i32, i32, i32), bool>, x: i32, y: i32, z: i32) -> u32 {
    let mut result = 0;
    for nx in x - 1..=x + 1 {
        for ny in y - 1..=y + 1 {
            for nz in z - 1..=z + 1 {
                if (nx, ny, nz) != (x, y, z) && *pocket.get(&(nx, ny, nz)).unwrap_or(&false) {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let mut pocket = HashMap::new();
    let (mut min_x, mut max_x) = (-1, 1);
    let (mut min_y, mut max_y) = (-1, 1);
    let (mut min_z, mut max_z) = (-1, 1);
    let (mut min_w, mut max_w) = (-1, 1);

    lines.iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            min_x = min(x as i32 - 1, min_x);
            min_y = min(y as i32 - 1, min_y);
            max_x = max(x as i32 + 1, max_x);
            max_y = max(y as i32 + 1, max_y);

            pocket.insert((x as i32, y as i32, 0i32, 0i32), c == '#');
        })
    });

    for _ in 0..6 {
        let last_pocket = pocket.clone();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for w in min_w..=max_w {
                        if *last_pocket.get(&(x, y, z, w)).unwrap_or(&false) {
                            match active_4d_neighbors(&last_pocket, x, y, z, w) {
                                2 | 3 => (),
                                _ => {
                                    pocket.insert((x, y, z, w), false);
                                }
                            }
                        } else {
                            match active_4d_neighbors(&last_pocket, x, y, z, w) {
                                3 => {
                                    min_x = min(x - 1, min_x);
                                    min_y = min(y - 1, min_y);
                                    min_z = min(z - 1, min_z);
                                    min_w = min(w - 1, min_w);
                                    max_x = max(x + 1, max_x);
                                    max_y = max(y + 1, max_y);
                                    max_z = max(z + 1, max_z);
                                    max_w = max(w + 1, max_w);

                                    pocket.insert((x, y, z, w), true);
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    pocket.values().filter(|&&v| v).count()
}

fn active_4d_neighbors(
    pocket: &HashMap<(i32, i32, i32, i32), bool>,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
) -> u32 {
    let mut result = 0;
    for nx in x - 1..=x + 1 {
        for ny in y - 1..=y + 1 {
            for nz in z - 1..=z + 1 {
                for nw in w - 1..=w + 1 {
                    if (nx, ny, nz, nw) != (x, y, z, w)
                        && *pocket.get(&(nx, ny, nz, nw)).unwrap_or(&false)
                    {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn _print_pocket(
    pocket: &HashMap<(i32, i32, i32), bool>,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    minz: i32,
    maxz: i32,
) {
    for z in minz..=maxz {
        println!("z={}", z);
        for y in miny..=maxy {
            for x in minx..=maxx {
                match *pocket.get(&(x, y, z)).unwrap_or(&false) {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = vec![".#.", "..#", "###"];

        assert_eq!(solve1(&lines), 112);
    }

    #[test]
    fn example2() {
        // Another slow one
        // TODO: improve algorithm if possible

        // let lines = vec![".#.", "..#", "###"];

        // assert_eq!(solve2(&lines), 848);
    }
}
