use std::collections::HashMap;

fn parse_input(input: String) -> Vec<u128> {
    let splits: Vec<&str> = input.split(",").collect();
    let mut result = vec![];
    splits.into_iter().for_each(|x| {
        result.push(x.parse::<u128>().unwrap());
    });
    result
}

fn get_min(crabs: &Vec<u128>) -> u128 {
    *crabs.into_iter().min().unwrap()
}

fn get_max(crabs: &Vec<u128>) -> u128 {
    *crabs.into_iter().max().unwrap()
}

fn calculate(crabs: Vec<u128>, calculate_fuel_cost: Box<dyn Fn(u128, u128) -> u128>) -> u128 {
    let mut position_tries: HashMap<u128, u128> = HashMap::new();
    for position in get_min(&crabs)..=get_max(&crabs) {
        position_tries.insert(
            position,
            crabs
                .clone()
                .into_iter()
                .fold(0, |acc, x| acc + calculate_fuel_cost(position, x)),
        );
    }
    *position_tries
        .clone()
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

pub fn part1(input: String) {
    println!(
        "{:?}",
        calculate(
            parse_input(input),
            Box::new(|position, x| (x as i64 - position as i64).abs() as u128),
        )
    ); // 331067
}

pub fn part2(input: String) {
    fn calculate_fuel_cost(a: u128, b: u128) -> u128 {
        let c = if a > b { a } else { b };
        let d = if a > b { b } else { a };
        match c - d {
            0 => 1,
            _ => (1..(c - d) + 1).sum(),
        }
    }
    println!(
        "{:?}",
        calculate(parse_input(input), Box::new(calculate_fuel_cost),)
    ); // 92881128
}
