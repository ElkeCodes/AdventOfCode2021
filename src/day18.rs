use std::{cmp::max, str::FromStr};

#[derive(Debug, Clone)]
enum SnailFishNumber {
    Value(u128),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

impl FromStr for SnailFishNumber {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.starts_with('[') {
            return Ok(SnailFishNumber::Value(input.parse::<u128>().unwrap()));
        }

        let mut cut_index = 0;
        let mut depth = 0;
        for (index, character) in input.chars().enumerate() {
            if character == '[' {
                depth += 1;
            } else if character == ']' {
                depth -= 1;
            }
            if depth == 1 && character == ',' {
                cut_index = index;
                break;
            }
        }

        let a = SnailFishNumber::from_str(&input[1..cut_index]);
        let b = SnailFishNumber::from_str(&input[cut_index + 1..input.len() - 1]);

        return Ok(SnailFishNumber::Pair(
            Box::new(a.unwrap()),
            Box::new(b.unwrap()),
        ));
    }
}

impl SnailFishNumber {
    fn magnitude(&self) -> u128 {
        match self {
            SnailFishNumber::Value(v) => *v,
            SnailFishNumber::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn explode(&mut self, depth: usize) -> (Option<u128>, Option<u128>, bool) {
        match self {
            SnailFishNumber::Pair(a, b) => {
                if depth >= 4 {
                    if let SnailFishNumber::Value(a_value) = **a {
                        if let SnailFishNumber::Value(b_value) = **b {
                            return (Some(a_value), Some(b_value), true);
                        }
                    }
                    panic!();
                }
                let (left_a, right_a, ea) = a.explode(depth + 1);
                if ea {
                    let mut l = None;
                    if left_a.is_some() && right_a.is_some() {
                        *a = Box::new(SnailFishNumber::Value(0));
                    }
                    if left_a.is_some() {
                        l = left_a;
                    }
                    if let Some(a_value) = right_a {
                        b.inject_left(a_value);
                    }
                    return (l, None, true);
                }

                let (left_b, right_b, eb) = b.explode(depth + 1);
                if eb {
                    let mut r = None;
                    if left_b.is_some() && right_b.is_some() {
                        *b = Box::new(SnailFishNumber::Value(0));
                    }
                    if let Some(b_value) = left_b {
                        a.inject_right(b_value);
                    }
                    if right_b.is_some() {
                        r = right_b;
                    }
                    return (None, r, true);
                }
                (None, None, false)
            }
            _ => (None, None, false),
        }
    }

    fn inject_left(&mut self, increment: u128) -> bool {
        match self {
            SnailFishNumber::Value(v) => {
                *v += increment;
                return true;
            }
            SnailFishNumber::Pair(a, _) => a.inject_left(increment),
        }
    }

    fn inject_right(&mut self, increment: u128) -> bool {
        match self {
            SnailFishNumber::Value(v) => {
                *v += increment;
                return true;
            }
            SnailFishNumber::Pair(_, b) => b.inject_right(increment),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailFishNumber::Value(v) => {
                if *v >= 10 {
                    *self = SnailFishNumber::Pair(
                        Box::new(SnailFishNumber::Value(*v / 2)),
                        Box::new(SnailFishNumber::Value(*v / 2 + *v % 2)),
                    );
                    return true;
                }
                return false;
            }
            SnailFishNumber::Pair(a, b) => {
                if a.split() {
                    return true;
                }
                return b.split();
            }
        }
    }
}

pub fn part1(input: String) {
    let snail_fish_numbers: Vec<SnailFishNumber> = input
        .lines()
        .map(SnailFishNumber::from_str)
        .map(|x| x.unwrap())
        .collect();
    println!(
        "{:?}",
        snail_fish_numbers
            .into_iter()
            .reduce(|previous_sum, current| {
                let mut current_number =
                    SnailFishNumber::Pair(Box::new(previous_sum), Box::new(current));
                let mut should_continue = true;
                while should_continue {
                    should_continue = current_number.explode(0).2;
                    if !should_continue {
                        should_continue = current_number.split();
                    }
                }
                current_number
            })
            .unwrap()
            .magnitude()
    ); // 3793
}

pub fn part2(input: String) {
    let snail_fish_numbers: Vec<SnailFishNumber> = input
        .lines()
        .map(SnailFishNumber::from_str)
        .map(|x| x.unwrap())
        .collect();
    let mut max_magnitude = 0;
    for i in 0..snail_fish_numbers.len() {
        for j in 0..snail_fish_numbers.len() {
            if i != j {
                let mut current_number = SnailFishNumber::Pair(
                    Box::new(snail_fish_numbers[i].clone()),
                    Box::new(snail_fish_numbers[j].clone()),
                );
                let mut should_continue = true;
                while should_continue {
                    should_continue = current_number.explode(0).2;
                    if !should_continue {
                        should_continue = current_number.split();
                    }
                }
                let mangiutde = current_number.magnitude();
                max_magnitude = max(max_magnitude, mangiutde);
            }
        }
    }
    println!("{:?}", max_magnitude); // 4695
}
