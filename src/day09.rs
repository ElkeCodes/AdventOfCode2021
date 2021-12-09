use std::cmp::{max, min};
use std::convert::TryInto;

fn get_lower_bound(x: usize) -> usize {
    max::<i32>(0, (x as i32 - 1).try_into().unwrap())
        .try_into()
        .unwrap()
}

fn get_upper_bound(x: usize, m: usize) -> usize {
    min(x + 1, m - 1)
}

fn is_lower_than_adjacents(depths: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let mut result = true;
    for test_y in get_lower_bound(y)..=get_upper_bound(y, depths.len()) {
        for test_x in get_lower_bound(x)..=get_upper_bound(x, depths[0].len()) {
            if y != test_y || x != test_x {
                result = result && depths[y][x] < depths[test_y][test_x];
            }
        }
    }
    result
}

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn part1(input: String) {
    let depths = parse_input(&input);

    let mut risk_level = 0;
    for (row_key, row) in depths.clone().into_iter().enumerate() {
        for (col_key, depth) in row.into_iter().enumerate() {
            if is_lower_than_adjacents(&depths, col_key, row_key) {
                risk_level += 1 + depth;
            }
        }
    }
    println!("{:?}", risk_level) // 560
}

// pub fn part2(input: String) {

// }
