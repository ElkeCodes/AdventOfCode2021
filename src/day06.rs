use std::collections::HashMap;

fn parse_input(input: String) -> HashMap<usize, u64> {
    let splits: Vec<&str> = input.split(",").collect();
    let mut result = HashMap::new();
    splits
        .into_iter()
        .for_each(|x| *(result.entry(x.parse::<usize>().unwrap()).or_insert(0)) += 1);
    result
}

fn run_days(fish: HashMap<usize, u64>, remaining_days: u64) -> u64 {
    if remaining_days == 0 {
        fish.iter().fold(0, |acc, (_, x)| acc + x)
    } else {
        let mut new_fish = HashMap::new();
        for (day, amount) in fish.into_iter() {
            if day == 0 {
                *new_fish.entry(6).or_insert(0) += amount;
                *new_fish.entry(8).or_insert(0) += amount;
            } else {
                *new_fish.entry(day - 1).or_insert(0) += amount;
            }
        }
        run_days(new_fish, remaining_days - 1)
    }
}

pub fn part1(input: String) {
    println!("{:?}", run_days(parse_input(input), 80)); // 391888
}

pub fn part2(input: String) {
    println!("{:?}", run_days(parse_input(input), 256)); // 1754597645339
}
