use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    ops::Add,
    vec,
};

fn parse_input(input: &String) -> (String, HashMap<String, String>) {
    let mut rules = HashMap::new();
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    lines.next();
    while let Some(insertion_rule) = lines.next() {
        let (key, value) = insertion_rule.split_once(" -> ").unwrap();
        rules.insert(key.to_string(), value.to_string());
    }
    (polymer_template.to_string(), rules)
}

pub fn part1(input: String) {
    fn step(polymer: String, rules: &HashMap<String, String>, steps_to_go: u8) -> String {
        if steps_to_go == 0 {
            polymer
        } else {
            let mut new_polymer = String::new().to_owned();
            for i in 0..polymer.len() - 1 {
                new_polymer.push_str(&polymer[i..i + 1]);
                if let Some(insertion) = rules.get(&polymer[i..=i + 1]) {
                    new_polymer.push_str(insertion);
                }
            }
            new_polymer.push_str(&polymer[polymer.len() - 1..polymer.len()]);
            step(new_polymer, rules, steps_to_go - 1)
        }
    }

    let (polymer, rules) = parse_input(&input);
    let new_polymer = step(polymer, &rules, 10);

    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in new_polymer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut most_common_element = counts.iter().nth(0).unwrap();
    let mut least_common_element = counts.iter().nth(0).unwrap();

    for (element, occurences) in &counts {
        if occurences > most_common_element.1 {
            most_common_element = (element, occurences);
        }
        if occurences < least_common_element.1 {
            least_common_element = (element, occurences);
        }
    }

    println!("{:?}", most_common_element.1 - least_common_element.1); // 3213
}

pub fn part2(input: String) {
    fn step(
        polymer: HashMap<String, u64>,
        rules: &HashMap<String, String>,
        steps_to_go: u8,
    ) -> HashMap<String, u64> {
        // println!("{:?}", polymer);
        if steps_to_go == 0 {
            polymer
        } else {
            let mut new_polymer = HashMap::new();
            for (s, n) in polymer.iter() {
                if let Some(insertion) = rules.get(s) {
                    let new_s_1 = String::from(&s[0..1]).add(insertion);
                    let new_s_2 = String::from(insertion).add(&s[1..2]);
                    *new_polymer.entry(new_s_1).or_insert(0) += n;
                    *new_polymer.entry(new_s_2).or_insert(0) += n;
                }
            }
            step(new_polymer, rules, steps_to_go - 1)
        }
    }
    let (polymer, rules) = parse_input(&input);
    let mut parsed_polymer: HashMap<String, u64> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        *parsed_polymer
            .entry(String::from(&polymer[i..=i + 1]))
            .or_insert(0) += 1;
    }
    let polymer_result = step(parsed_polymer, &rules, 40);

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
    println!("{:?}", most_common_element.1 - least_common_element.1 + 1); // 3213
}
