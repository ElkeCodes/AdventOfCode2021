use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::VecDeque,
    convert::TryInto,
};

#[derive(Clone)]
struct Octopus {
    light_level: u32,
    last_step_flashed: u32,
}

fn get_lower_bound(x: usize) -> usize {
    max::<i32>(0, (x as i32 - 1).try_into().unwrap())
        .try_into()
        .unwrap()
}

fn get_upper_bound(x: usize, m: usize) -> usize {
    min(x + 1, m - 1)
}

fn parse_input(input: &String) -> Vec<Vec<Octopus>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| Octopus {
                    light_level: x.to_digit(10).unwrap(),
                    last_step_flashed: 0,
                })
                .collect::<Vec<Octopus>>()
        })
        .collect::<Vec<Vec<Octopus>>>()
}

fn perform_step(octopuses: &mut Vec<Vec<Octopus>>, step: u32) -> u32 {
    let mut to_visit = (0..10)
        .cartesian_product(0..10)
        .collect::<VecDeque<(usize, usize)>>();
    let mut flashes = 0;
    while to_visit.len() > 0 {
        let (x, y) = to_visit.pop_front().unwrap();
        let mut octopus = octopuses[y][x].clone();
        octopus.light_level += 1;
        if octopus.last_step_flashed < step && octopus.light_level > 9 {
            flashes += 1;
            for test_y in get_lower_bound(y)..=get_upper_bound(y, 10) {
                for test_x in get_lower_bound(x)..=get_upper_bound(x, 10) {
                    if octopuses[test_y][test_x].light_level <= 9
                        && !(test_x == x && test_y == y)
                        && octopuses[test_y][test_x].last_step_flashed != step
                    {
                        to_visit.push_back((test_x, test_y));
                    }
                }
            }
            octopuses[y][x] = Octopus {
                light_level: 0,
                last_step_flashed: step,
            };
        } else if octopus.last_step_flashed < step {
            octopuses[y][x] = Octopus {
                light_level: octopus.light_level,
                last_step_flashed: octopus.last_step_flashed,
            };
        }
    }
    flashes
}

pub fn part1(input: String) {
    let mut octopuses = parse_input(&input);
    let mut step = 0;
    let mut flashes = 0;
    while step <= 99 {
        step += 1;
        flashes += perform_step(&mut octopuses, step);
    }
    println!("{:?}", flashes) // 1739
}

pub fn part2(input: String) {
    let mut octopuses = parse_input(&input);
    let mut step = 0;
    while perform_step(&mut octopuses, step) <= 99 {
        step += 1;
    }
    println!("{:?}", step) // 324
}
