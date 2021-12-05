use std::collections::HashMap;

use itertools::Itertools;

type Coordinate = (i16, i16);
type Delta = (i16, i16);

fn parse_coordinate(coordinate: &str) -> Coordinate {
    return (
        coordinate
            .split(",")
            .nth(0)
            .unwrap()
            .parse::<i16>()
            .unwrap(),
        coordinate
            .split(",")
            .nth(1)
            .unwrap()
            .parse::<i16>()
            .unwrap(),
    );
}

fn get_delta(x1: i16, x2: i16) -> i16 {
    if x1 < x2 {
        1
    } else if x1 > x2 {
        -1
    } else {
        0
    }
}

fn get_coordinates_delta(a: Coordinate, b: Coordinate) -> Delta {
    (get_delta(a.0, b.0), get_delta(a.1, b.1))
}

fn parse_lines(input: String) -> Vec<(Coordinate, Coordinate, Delta)> {
    let mut field = vec![];
    for input_line in input.lines() {
        let splits: Vec<&str> = input_line.split(" -> ").collect();
        let a = parse_coordinate(splits.iter().sorted().nth(0).cloned().unwrap());
        let b = parse_coordinate(splits.iter().sorted().rev().nth(0).cloned().unwrap());
        let delta = get_coordinates_delta(a, b);
        field.push((a, b, delta));
    }

    return field;
}

fn generate_map(
    coordinates: Vec<(Coordinate, Coordinate, Delta)>,
    filter: Box<dyn Fn(&Coordinate, &Coordinate) -> bool>,
) -> HashMap<Coordinate, i16> {
    let mut field: HashMap<Coordinate, i16> = HashMap::new();
    fn insert_into_field(field: &mut HashMap<Coordinate, i16>, coordinate: Coordinate) {
        let occurences = field.entry(coordinate).or_insert(0);
        *occurences += 1;
    }

    for (mut a, b, delta) in coordinates.into_iter() {
        if filter(&a, &b) {
            loop {
                let should_break = a.0 == b.0 && a.1 == b.1;
                insert_into_field(&mut field, a);
                a.0 += delta.0;
                a.1 += delta.1;
                if should_break {
                    break;
                }
            }
        }
    }
    return field;
}

pub fn part1(input: String) {
    let field = generate_map(parse_lines(input), Box::new(|a, b| a.0 == b.0 || a.1 == b.1));
    let result = field.iter().filter(|(_, &value)| value >= 2).count();
    println!("{:?}", result);
}

pub fn part2(input: String) {
    let field = generate_map(parse_lines(input), Box::new(|_, _| true));
    let result = field.iter().filter(|(_, &value)| value >= 2).count();
    println!("{:?}", result);
}
