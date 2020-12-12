pub fn solve1(lines: &Vec<&str>) -> usize {
    let mut ship = Ship::new();

    for line in lines {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let value = chars.as_str().parse::<i32>().unwrap();

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

    ship.manhattan_distance()
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    0
}

struct Ship {
    x: i32,
    y: i32,
    heading: u16,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 90,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = vec!["F10", "N3", "F7", "R90", "F11"];

        assert_eq!(solve1(&lines), 25);
    }
}
