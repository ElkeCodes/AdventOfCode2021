use std::{
    cmp::{max, min},
    str::FromStr,
};

#[derive(Debug)]
struct Target {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl FromStr for Target {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input
            .strip_prefix("target area: x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();
        let (temp_x1, temp_x2) = x.split_once("..").unwrap();
        let x1 = isize::from_str(temp_x1).unwrap();
        let x2 = isize::from_str(temp_x2).unwrap();
        let (temp_y1, temp_y2) = y.split_once("..").unwrap();
        let y1 = isize::from_str(temp_y1).unwrap();
        let y2 = isize::from_str(temp_y2).unwrap();
        Ok(Target {
            min_x: min(x1, x2),
            max_x: max(x1, x2),
            min_y: min(y1, y2),
            max_y: max(y1, y2),
        })
    }
}

#[derive(Debug)]
struct Velocity {
    x: isize,
    y: isize,
}

impl Velocity {
    fn drag(&mut self) {
        self.y -= 1;
        self.x += if self.x != 0 {
            self.x.abs() / -self.x
        } else {
            0
        };
    }
}

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn step(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
    }
    fn hits_target(&mut self, target: &Target) -> bool {
        target.min_x <= self.x
            && target.max_x >= self.x
            && target.min_y <= self.y
            && target.max_y >= self.y
    }
    fn has_overshot_target(&mut self, target: &Target) -> bool {
        target.max_x < self.x || target.min_y > self.y
    }
}

fn perform_hit(target: &Target, mut velocity: Velocity) -> Option<isize> {
    let mut coordinate = Coordinate { x: 0, y: 0 };
    let mut highest_y = coordinate.y;
    while !coordinate.has_overshot_target(&target) {
        coordinate.step(&velocity);
        highest_y = max(coordinate.y, highest_y);
        if coordinate.hits_target(&target) {
            return Some(highest_y);
        }
        velocity.drag();
    }
    None
}

pub fn part1(input: String) {
    match Target::from_str(&input) {
        Ok(target) => {
            let mut max_y = -target.min_y;
            for x in 1..=target.max_x {
                for y in -target.min_y.abs()..=target.min_y.abs() {
                    if let Some(highest_y) = perform_hit(&target, Velocity { x, y }) {
                        max_y = max(max_y, highest_y);
                    }
                }
            }
            println!("{:?}", max_y); // 10878
        }
        Err(_) => {}
    };
}

pub fn part2(input: String) {
    match Target::from_str(&input) {
        Ok(target) => {
            let mut amount_good_velocities = 0;
            for x in 1..=target.max_x {
                for y in -target.min_y.abs()..=target.min_y.abs() {
                    if perform_hit(&target, Velocity { x, y }).is_some() {
                        amount_good_velocities += 1;
                    }
                }
            }
            println!("{:?}", amount_good_velocities); // 4716
        }
        Err(_) => {}
    };
}
