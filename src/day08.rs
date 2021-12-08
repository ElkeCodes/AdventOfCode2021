use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: String) {
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let (_, output) = line.split_once(" | ").unwrap();
                output.split_whitespace().collect::<Vec<_>>()
            })
            .map(|output| output
                .into_iter()
                .filter(|word| match word.len() {
                    2 => true,
                    3 => true,
                    4 => true,
                    7 => true,
                    _ => false,
                })
                .count())
            .sum::<usize>()
    )
}

pub fn part2(input: String) {
    fn not_compares(mappings: &HashMap<u32, HashSet<char>>, other: u32, x: &str) -> bool {
        mappings.contains_key(&other)
            && *mappings.get(&other).unwrap() != x.chars().collect::<HashSet<_>>()
    }
    fn contains(mappings: &HashMap<u32, HashSet<char>>, other: u32, x: &str) -> bool {
        mappings.contains_key(&other)
            && (*mappings.get(&other).unwrap()).is_subset(&x.chars().collect::<HashSet<_>>())
    }
    fn reverse_contains(mappings: &HashMap<u32, HashSet<char>>, other: u32, x: &str) -> bool {
        mappings.contains_key(&other)
            && (*mappings.get(&other).unwrap()).is_superset(&x.chars().collect::<HashSet<_>>())
    }
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let (input, output) = line.split_once(" | ").unwrap();
                (
                    input.split_whitespace().collect::<VecDeque<_>>(),
                    output.split_whitespace().clone().collect::<Vec<_>>(),
                )
            })
            .map(|(mut input, output)| {
                let mut mappings: HashMap<u32, HashSet<char>> = HashMap::new();
                while input.len() > 0 {
                    let word = input.pop_front().unwrap();
                    match word.len() {
                        2 => {
                            mappings.insert(1, word.chars().collect());
                        }
                        3 => {
                            mappings.insert(7, word.chars().collect());
                        }
                        4 => {
                            mappings.insert(4, word.chars().collect());
                        }
                        7 => {
                            mappings.insert(8, word.chars().collect());
                        }
                        5 => {
                            if contains(&mappings, 1, word) {
                                mappings.insert(3, word.chars().collect());
                            } else if not_compares(&mappings, 3, word)
                                && reverse_contains(&mappings, 9, word)
                            {
                                mappings.insert(5, word.chars().collect());
                            } else if not_compares(&mappings, 3, word)
                                && not_compares(&mappings, 5, word)
                            {
                                mappings.insert(2, word.chars().collect());
                            } else {
                                input.push_back(word);
                            }
                        }
                        6 => {
                            if contains(&mappings, 4, word) {
                                mappings.insert(9, word.chars().collect());
                            } else if not_compares(&mappings, 9, word)
                                && contains(&mappings, 1, word)
                            {
                                mappings.insert(0, word.chars().collect());
                            } else if not_compares(&mappings, 9, word)
                                && not_compares(&mappings, 0, word)
                            {
                                mappings.insert(6, word.chars().collect());
                            } else {
                                input.push_back(word);
                            }
                        }
                        _ => {}
                    }
                }
                let mut result: u64 = 0;
                for word in output.into_iter() {
                    result = result * 10
                        + mappings
                            .iter()
                            .find_map(|(key, val)| {
                                if *val == word.chars().collect::<HashSet<_>>() {
                                    Some(*key)
                                } else {
                                    None
                                }
                            })
                            .unwrap() as u64;
                }
                result
            })
            .sum::<u64>()
    );
}
