use itertools::Itertools;

fn parse_input(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn parse_line(line: &Vec<char>) -> (u32, Vec<&char>, u64) {
    fn get_score(c: char) -> u32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }
    fn get_incomplete_score(mut incomplete_chars: Vec<char>) -> u64 {
        let mut result = 0;
        while incomplete_chars.len() > 0 {
            result *= 5;
            result += match incomplete_chars.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        result
    }
    let mut parsed_chars = vec![];
    let mut to_check: Vec<&char> = line.iter().rev().collect();
    let mut syntax_error_score = 0;
    while to_check.len() > 0 {
        let first = *to_check.pop().unwrap();
        match first {
            '<' | '[' | '(' | '{' => parsed_chars.push(first),
            '>' => {
                if *parsed_chars.last().unwrap() == '<' {
                    parsed_chars.pop();
                } else {
                    syntax_error_score = get_score(first);
                    break;
                }
            }
            ']' => {
                if *parsed_chars.last().unwrap() == '[' {
                    parsed_chars.pop();
                } else {
                    syntax_error_score = get_score(first);
                    break;
                }
            }
            ')' => {
                if *parsed_chars.last().unwrap() == '(' {
                    parsed_chars.pop();
                } else {
                    syntax_error_score = get_score(first);
                    break;
                }
            }
            '}' => {
                if *parsed_chars.last().unwrap() == '{' {
                    parsed_chars.pop();
                } else {
                    syntax_error_score = get_score(first);
                    break;
                }
            }
            _ => {}
        }
    }
    (
        syntax_error_score,
        to_check.clone(),
        if to_check.len() == 0 {
            get_incomplete_score(parsed_chars)
        } else {
            0
        },
    )
}

pub fn part1(input: String) {
    let mut result = 0;
    for line in parse_input(&input) {
        result += parse_line(&line).0;
    }
    println!("{:?}", result) // 413733
}

pub fn part2(input: String) {
    let mut incomplete_scores = vec![];
    for line in parse_input(&input) {
        let incomplete_score = parse_line(&line).2;
        if incomplete_score > 0 {
            incomplete_scores.push(incomplete_score);
        }
    }
    let middle = (incomplete_scores.len() as f64 / 2 as f64).floor() as usize;
    println!(
        "{:?}",
        incomplete_scores.iter().sorted().nth(middle).unwrap()
    ) // 3354640192
}
