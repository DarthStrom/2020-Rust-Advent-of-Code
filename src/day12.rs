pub fn solve1(lines: &Vec<&str>) -> usize {
    solve(lines, &guess)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    solve(lines, &instructions)
}

fn solve(lines: &Vec<&str>, strategy: &dyn Fn(&mut Ship, char, i32)) -> usize {
    let mut ship = Ship::new();

    for line in lines {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let value = chars.as_str().parse::<i32>().unwrap();

        strategy(&mut ship, action, value);
    }

    ship.manhattan_distance()
}

fn guess(ship: &mut Ship, action: char, value: i32) {
    match action {
        'N' => ship.move_n(value),
        'S' => ship.move_s(value),
        'E' => ship.move_e(value),
        'W' => ship.move_w(value),
        'L' => ship.move_l(value),
        'R' => ship.move_r(value),
        'F' => ship.move_f(value),
        _ => panic!("unknown action"),
    }
}

fn instructions(ship: &mut Ship, action: char, value: i32) {
    match action {
        'N' => ship.waypoint_n(value),
        'S' => ship.waypoint_s(value),
        'E' => ship.waypoint_e(value),
        'W' => ship.waypoint_w(value),
        'L' => ship.rotate_l(value),
        'R' => ship.rotate_r(value),
        'F' => ship.move_to_waypoint(value),
        _ => panic!("unknown action"),
    }
}

struct Ship {
    x: i32,
    y: i32,
    heading: u16,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 90,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }
}

impl Ship {
    fn new() -> Self {
        Self::default()
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn move_n(&mut self, value: i32) {
        self.y += value;
    }

    fn move_s(&mut self, value: i32) {
        self.y -= value;
    }

    fn move_e(&mut self, value: i32) {
        self.x += value;
    }

    fn move_w(&mut self, value: i32) {
        self.x -= value;
    }

    fn move_l(&mut self, value: i32) {
        self.heading = ((360 + self.heading as i32 - value) % 360) as u16;
    }

    fn move_r(&mut self, value: i32) {
        self.heading = ((self.heading as i32 + value) % 360) as u16;
    }

    fn move_f(&mut self, value: i32) {
        match self.heading {
            0 => self.move_n(value),
            90 => self.move_e(value),
            180 => self.move_s(value),
            270 => self.move_w(value),
            _ => unimplemented!(),
        }
    }

    fn waypoint_n(&mut self, value: i32) {
        self.waypoint_y += value;
    }

    fn waypoint_s(&mut self, value: i32) {
        self.waypoint_y -= value;
    }

    fn waypoint_e(&mut self, value: i32) {
        self.waypoint_x += value;
    }

    fn waypoint_w(&mut self, value: i32) {
        self.waypoint_x -= value;
    }

    fn flip(&mut self) {
        self.waypoint_x = -self.waypoint_x;
        self.waypoint_y = -self.waypoint_y;
    }

    fn rotate_l(&mut self, value: i32) {
        match value {
            90 => {
                let new_y = self.waypoint_x;
                self.waypoint_x = -self.waypoint_y;
                self.waypoint_y = new_y;
            }
            180 => self.flip(),
            270 => self.rotate_r(90),
            _ => unimplemented!(),
        }
    }

    fn rotate_r(&mut self, value: i32) {
        match value {
            90 => {
                let new_y = -self.waypoint_x;
                self.waypoint_x = self.waypoint_y;
                self.waypoint_y = new_y;
            }
            180 => self.flip(),
            270 => self.rotate_l(90),
            _ => unimplemented!(),
        }
    }

    fn move_to_waypoint(&mut self, value: i32) {
        self.x += value * self.waypoint_x;
        self.y += value * self.waypoint_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 5] = ["F10", "N3", "F7", "R90", "F11"];

    #[test]
    fn example1() {
        assert_eq!(solve1(&LINES.to_vec()), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(&LINES.to_vec()), 286);
    }
}
