use std::{cmp::max, collections::HashSet, vec};

type Grid = HashSet<(u32, u32)>;

fn parse_input(input: &String) -> (Grid, Vec<(char, u32)>) {
    let mut result = HashSet::new();
    let mut fold_instructions = vec![];
    input.lines().for_each(|line| {
        if line.contains(",") {
            let new_x = line.split_once(",").unwrap().0.parse::<u32>().unwrap();
            let new_y = line.split_once(",").unwrap().1.parse::<u32>().unwrap();
            result.insert((new_x, new_y));
        } else if line.contains("fold along x=") {
            fold_instructions.push(('x', line.split_once("=").unwrap().1.parse::<u32>().unwrap()));
        } else if line.contains("fold along y=") {
            fold_instructions.push(('y', line.split_once("=").unwrap().1.parse::<u32>().unwrap()));
        }
    });
    (result, fold_instructions)
}

fn fold_along_x(grid: &mut Grid, x: u32) {
    for (test_x, test_y) in grid.clone().into_iter() {
        if test_x > x {
            grid.remove(&(test_x, test_y));
            let new_x = x - (test_x - x);
            grid.insert((new_x, test_y));
        }
    }
}

fn fold_along_y(grid: &mut Grid, y: u32) {
    for (test_x, test_y) in grid.clone().into_iter() {
        if test_y > y {
            grid.remove(&(test_x, test_y));
            let new_y = y - (test_y - y);
            grid.insert((test_x, new_y));
        }
    }
}

pub fn part1(input: String) {
    let (mut grid, fold_instructions) = parse_input(&input);
    let (fold_axis, n) = fold_instructions.first().unwrap();
    match fold_axis {
        'x' => fold_along_x(&mut grid, *n),
        _ => fold_along_y(&mut grid, *n),
    }
    println!("{:?}", grid.into_iter().count()) // 669
}

pub fn part2(input: String) {
    let (mut grid, fold_instructions) = parse_input(&input);
    for (fold_axis, n) in fold_instructions.into_iter() {
        match fold_axis {
            'x' => fold_along_x(&mut grid, n),
            _ => fold_along_y(&mut grid, n),
        }
    }
    let max_x = grid.clone().into_iter().fold(0, |acc, (x, _)| max(acc, x));
    let max_y = grid.clone().into_iter().fold(0, |acc, (_, y)| max(acc, y));

    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    } // UEFZCUCJ
}
