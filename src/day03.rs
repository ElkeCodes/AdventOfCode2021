use itertools::Itertools;
use std::collections::HashMap;

fn calculate_occurences_of_ones(input: &str) -> HashMap<usize, usize> {
    let input_length = input.lines().nth(0).unwrap().len();
    let mut occurences_of_ones = HashMap::new();
    for line in input.lines() {
        for index in 0..input_length {
            let amount = occurences_of_ones.entry(index).or_insert(0);
            if line.get(index..index + 1).unwrap() == "1" {
                *amount += 1;
            }
        }
    }
    return occurences_of_ones;
}

pub fn part1(input: String) {
    let occurences_of_ones = calculate_occurences_of_ones(&input);
    let amount_of_inputs = input.lines().count();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for (_, &entry) in occurences_of_ones.iter().sorted_by_key(|x| x.0) {
        if entry > amount_of_inputs - entry {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }
    println!(
        "{:?}",
        i32::from_str_radix(&gamma_rate, 2).unwrap()
            * i32::from_str_radix(&epsilon_rate, 2).unwrap()
    );
}

pub fn part2(input: String) {
    fn get_amount_ones(candidates: &Vec<&str>, index: usize) -> usize {
        return candidates
            .iter()
            .filter(|x| x.as_bytes()[index] == b'1')
            .count();
    }
    fn find_rate(input: &str, focus_bit: u8, rest_bit: u8) -> usize {
        let mut candidates: Vec<&str> = input.lines().collect();
        let mut index = 0;
        while candidates.len() > 1 {
            let amount_ones = get_amount_ones(&candidates, index);
            if amount_ones >= (candidates.len() - amount_ones) {
                candidates = candidates
                    .into_iter()
                    .filter(|x| x.as_bytes()[index] == focus_bit)
                    .collect();
            } else {
                candidates = candidates
                    .into_iter()
                    .filter(|x| x.as_bytes()[index] == rest_bit)
                    .collect();
            }
            index += 1;
        }
        return usize::from_str_radix(candidates.first().unwrap(), 2).unwrap();
    }
    println!(
        "{:?}",
        find_rate(&input, b'1', b'0') * find_rate(&input, b'0', b'1')
    );
}
