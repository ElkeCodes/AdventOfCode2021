use std::{collections::HashMap, ops::Add};

type Polymer = HashMap<String, u64>;
type Rules = HashMap<String, String>;

fn parse_input(input: &String) -> (Polymer, Rules) {
    let mut rules = Rules::new();
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    lines.next();
    while let Some(insertion_rule) = lines.next() {
        let (key, value) = insertion_rule.split_once(" -> ").unwrap();
        rules.insert(key.to_string(), value.to_string());
    }

    let mut parsed_polymer = Polymer::new();
    let polymer = polymer_template.to_string();
    for i in 0..polymer.len() - 1 {
        *parsed_polymer
            .entry(String::from(&polymer[i..=i + 1]))
            .or_insert(0) += 1;
    }
    (parsed_polymer, rules)
}

fn step(polymer: Polymer, rules: &Rules, steps: u8) -> Polymer {
    if steps == 0 {
        polymer
    } else {
        let mut new_polymer = Polymer::new();
        for (s, n) in polymer.iter() {
            if let Some(insertion) = rules.get(s) {
                let new_s_1 = String::from(&s[0..1]).add(insertion);
                let new_s_2 = String::from(insertion).add(&s[1..2]);
                *new_polymer.entry(new_s_1).or_insert(0) += n;
                *new_polymer.entry(new_s_2).or_insert(0) += n;
            }
        }
        step(new_polymer, rules, steps - 1)
    }
}

fn calculate_result(polymer_result: Polymer) -> u64 {
    let mut parsed_results = HashMap::new();
    for (element, occurences) in &polymer_result {
        let first_char = element.chars().nth(0).unwrap();
        *parsed_results.entry(first_char).or_insert(0) += occurences;
    }
    let (last_element, _) = polymer_result.iter().last().unwrap();
    *parsed_results
        .entry(last_element.chars().nth(1).unwrap())
        .or_insert(0) += 1;

    let mut most_common_element = parsed_results.iter().nth(0).unwrap();
    let mut least_common_element = parsed_results.iter().nth(0).unwrap();
    for (element, occurences) in &parsed_results {
        if occurences > most_common_element.1 {
            most_common_element = (element, occurences);
        }
        if occurences < least_common_element.1 {
            least_common_element = (element, occurences);
        }
    }
    most_common_element.1 - least_common_element.1 + 1
}

pub fn part1(input: String) {
    let (polymer, rules) = parse_input(&input);
    let polymer_result = step(polymer, &rules, 10);

    println!("{:?}", calculate_result(polymer_result)); // 3213
}

pub fn part2(input: String) {
    let (polymer, rules) = parse_input(&input);
    let polymer_result = step(polymer, &rules, 40);

    println!("{:?}", calculate_result(polymer_result)); // 3711743744429
}
