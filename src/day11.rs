use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::VecDeque,
    convert::TryInto,
};

type Octopus = (u32, u32);

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
                .map(|x| (x.to_digit(10).unwrap(), 0))
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
        let mut octopus = octopuses[y][x];
        octopus.0 += 1;
        if octopus.1 < step && octopus.0 > 9 {
            flashes += 1;
            for test_y in get_lower_bound(y)..=get_upper_bound(y, 10) {
                for test_x in get_lower_bound(x)..=get_upper_bound(x, 10) {
                    if octopuses[test_y][test_x].0 <= 9
                        && !(test_x == x && test_y == y)
                        && octopuses[test_y][test_x].1 != step
                    {
                        to_visit.push_back((test_x, test_y));
                    }
                }
            }
            octopuses[y][x] = (0, step);
        } else if octopus.1 < step {
            octopuses[y][x] = (octopus.0, octopus.1);
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
