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

pub fn part2(input: String) {
    fn grow_basin(depths: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
        let mut result = 0;
        let mut to_visit = vec![];
        for test_y in get_lower_bound(y)..=get_upper_bound(y, depths.len()) {
            for test_x in get_lower_bound(x)..=get_upper_bound(x, depths[0].len()) {
                if (test_y == y || test_x == x) && depths[test_y][test_x] != 9 {
                    result += 1;
                    depths[test_y][test_x] = 9;
                    to_visit.push((test_x, test_y));
                }
            }
        }
        if to_visit.len() > 0 {
            for to in to_visit.into_iter() {
                result += grow_basin(depths, to.0, to.1);
            }
        }
        result
    }

    let mut depths = parse_input(&input);

    let mut lowest_points: Vec<(usize, usize)> = vec![];
    for (row_key, row) in depths.clone().into_iter().enumerate() {
        for (col_key, _) in row.into_iter().enumerate() {
            if is_lower_than_adjacents(&depths, col_key, row_key) {
                &lowest_points.push((col_key, row_key));
            }
        }
    }

    let mut found_basins = vec![];
    for basin_start in lowest_points.clone().into_iter() {
        found_basins.push((
            basin_start.0,
            basin_start.1,
            grow_basin(&mut depths, basin_start.0, basin_start.1),
        ));
    }
    found_basins.sort_by(|a, b| b.2.cmp(&a.2));
    println!(
        "{:?}",
        found_basins[0].2 * found_basins[1].2 * found_basins[2].2
    ) // 959136
}
