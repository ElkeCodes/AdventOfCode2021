use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    vec, fs::File, io::Write,
};

use itertools::Itertools;

type Grid = HashMap<(u32, u32), u8>;

fn parse_input(input: &String) -> (Grid, Vec<(char, u32)>, u32, u32) {
    let mut result = HashMap::new();
    let mut fold_instructions = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    input.lines().for_each(|line| {
        if line.contains(",") {
            let new_x = line.split_once(",").unwrap().0.parse::<u32>().unwrap();
            let new_y = line.split_once(",").unwrap().1.parse::<u32>().unwrap();
            result.insert((new_x, new_y), 1);
            max_x = max(max_x, new_x);
            max_y = max(max_y, new_y);
        } else if line.contains("fold along x=") {
            fold_instructions.push(('x', line.split_once("=").unwrap().1.parse::<u32>().unwrap()));
        } else if line.contains("fold along y=") {
            fold_instructions.push(('y', line.split_once("=").unwrap().1.parse::<u32>().unwrap()));
        }
    });
    (result, fold_instructions, max_x, max_y)
}

fn fold_along_x(grid: &mut Grid, x: u32, max_x: u32) {
    for (key, value) in grid.clone().into_iter() {
        if key.0 > x {
            let new_x = max_x - key.0;
            grid.entry((new_x, key.1))
                .and_modify(|n| *n += 1)
                .or_insert(1);
            grid.remove_entry(&key);
        }
    }
}

fn fold_along_y(grid: &mut Grid, y: u32, max_y: u32) {
    for (key, value) in grid.clone().into_iter() {
        if key.1 > y {
            let new_y = max_y - key.1;
            grid.entry((key.0, new_y))
                .and_modify(|n| *n += 1)
                .or_insert(1);
            grid.remove_entry(&key);
        }
    }
}

pub fn part1(input: String) {
    let (mut grid, fold_instructions, max_x, max_y) = parse_input(&input);
    let (fold_axis, n) = fold_instructions.first().unwrap();
    match fold_axis {
        'x' => fold_along_x(&mut grid, *n, max_x),
        'y' => fold_along_y(&mut grid, *n, max_y),
        _ => {}
    }
    println!("{:?}", grid.into_iter().count()) // 669
}

pub fn part2(input: String) {
    let (mut grid, fold_instructions, max_x, max_y) = parse_input(&input);
    for (fold_axis, n) in fold_instructions.into_iter() {
        match fold_axis {
            'x' => fold_along_x(&mut grid, n, max_x),
            'y' => fold_along_y(&mut grid, n, max_y),
            _ => {}
        }
    }
    // for row in grid.into_iter().fold(0, |acc, x| max(acc, x)) {
    //     println!("{:?}", row);
    // }
    let max_x = grid
        .clone()
        .into_iter()
        .fold(0, |acc, ((x, y), n)| max(acc, x));
    let max_y = grid
        .clone()
        .into_iter()
        .fold(0, |acc, ((x, y), n)| max(acc, y));
    let min_x = grid
        .clone()
        .into_iter()
        .fold(max_x, |acc, ((x, y), n)| min(acc, x));
    let min_y = grid
        .clone()
        .into_iter()
        .fold(max_y, |acc, ((x, y), n)| min(acc, y));
    println!("{:?} {:?} {:?} {:?}", max_x, max_y, min_x, min_y);

    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    // let mut s = String::new();
    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if grid.contains_key(&(x, y)) {
    //             s.push('#');
    //         } else {
    //             s.push('.');
    //         }
    //     }

    //     s.push('\n');
    // }
	// // print!("{:?}", s);
	// let mut file = File::create("foo.txt").unwrap();
    // file.write(s.as_bytes());
    // println!("{:?}", grid.into_iter().count()) // 669
}
