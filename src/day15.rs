use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap},
    convert::TryInto,
    vec,
};

type Coordinate = (usize, usize);
type Grid = HashMap<Coordinate, u32>;
type Path = Vec<Coordinate>;

fn get_lower_bound(x: usize) -> usize {
    max::<i32>(0, (x as i32 - 1).try_into().unwrap())
        .try_into()
        .unwrap()
}

fn get_upper_bound(x: usize, m: usize) -> usize {
    min(x + 1, m)
}

fn parse_input(input: &String) -> (Grid, Coordinate) {
    let mut result = Grid::new();
    let mut end_x = 0;
    let mut end_y = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        if y > end_y {
            end_y = y;
        }
        for (x, risk) in line.chars().enumerate() {
            result.insert((x, y), risk.to_digit(10).unwrap().try_into().unwrap());
            if x > end_x {
                end_x = x;
            }
        }
    });
    (result, (end_x, end_y))
}

fn find_path(grid: &Grid, start: Coordinate, end: Coordinate) -> Option<Path> {
    fn get_neighbours(coordinate: &Coordinate, end: &Coordinate) -> Vec<Coordinate> {
        let mut unvisited = vec![];
        for test_y in get_lower_bound(coordinate.1)..=get_upper_bound(coordinate.1, end.1) {
            for test_x in get_lower_bound(coordinate.0)..=get_upper_bound(coordinate.0, end.0) {
                if (test_x == coordinate.0 || test_y == coordinate.1)
                    && !(test_x == coordinate.0 && test_y == coordinate.1)
                {
                    unvisited.push((test_x, test_y));
                }
            }
        }
        unvisited
    }
    let mut dist: HashMap<Coordinate, (u32, Option<Coordinate>)> = HashMap::new();
    dist.insert(start, (0, None));
    let mut heap: BinaryHeap<(Coordinate, u32)> = BinaryHeap::new();
    heap.push((start, 0));
    let mut result: Option<Vec<Coordinate>> = None;
    while let Some((current, cost)) = heap.pop() {
        if current == end {
            let mut path = vec![];
            path.push(end);
            let mut current_dist = dist.get(&end).unwrap();
            while let Some(previous) = current_dist.1 {
                path.push(previous);
                current_dist = dist.get(&previous).unwrap();
            }
            result = Some(path);
        }

        if cost > dist.entry(current).or_insert((u32::MAX, None)).0 {
            continue;
        }

        for neighbour in get_neighbours(&current, &end).iter() {
            let new_cost = *grid.get(&neighbour).unwrap() + cost;
            if new_cost < dist.entry(*neighbour).or_insert((u32::MAX, None)).0 {
                dist.insert(neighbour.clone(), (new_cost, Some(current)));
                heap.push((neighbour.clone(), new_cost));
            }
        }
    }
    result
}

fn calculate_risk_level(grid: &Grid, path: &Path) -> u32 {
    path.iter()
        .fold(0, |acc, coordinate| acc + grid.get(coordinate).unwrap())
        - grid.get(path.iter().last().unwrap()).unwrap()
}

pub fn part1(input: String) {
    let (grid, end_coordinate) = parse_input(&input);
    let path = find_path(&grid, (0, 0), end_coordinate);
    println!("{:?}", calculate_risk_level(&grid, &path.unwrap()));
}

pub fn part2(input: String) {
    fn calculate_new_risk(risk: u32, times_x: usize, times_y: usize) -> u32 {
        let new_risk = risk + times_x as u32 + times_y as u32;
        if new_risk > 9 {
            new_risk % 10 + 1
        } else {
            new_risk
        }
    }
    let (grid, end_coordinate) = parse_input(&input);
    let mut new_grid = Grid::new();
    for ((x, y), risk) in grid.clone().into_iter() {
        for times_y in 0..5 {
            for times_x in 0..5 {
                new_grid.insert(
                    (x + 100 * times_x, y + 100 * times_y),
                    calculate_new_risk(risk, times_x, times_y),
                );
            }
        }
    }
    let path = find_path(
        &new_grid,
        (0, 0),
        (end_coordinate.0 + 400, end_coordinate.1 + 400),
    );
    println!("{:?}", calculate_risk_level(&new_grid, &path.unwrap()));
}
