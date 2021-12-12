use std::{
    collections::{HashMap, HashSet},
    vec,
};

type Grid = HashMap<String, HashSet<String>>;

fn parse_input(input: &String) -> Grid {
    let mut result = HashMap::new();
    input.lines().for_each(|line| {
        let (from, to) = line.split_once("-").unwrap();
        (*result.entry(String::from(from)).or_insert(HashSet::new())).insert(String::from(to));
        (*result.entry(String::from(to)).or_insert(HashSet::new())).insert(String::from(from));
    });
    result
}

pub fn part1(input: String) {
    fn find_distinct_path(grid: &Grid, start: String, mut visited: Vec<String>) -> u32 {
        if start == "end" {
            1
        } else {
            let entry = grid.get(&start).unwrap();
            visited.append(&mut vec![start.to_string()]);
            let mut n = 0;
            for new_node in entry
                .clone()
                .into_iter()
                .filter(|x| x.to_lowercase() != *x || !visited.contains(&x))
            {
                n += find_distinct_path(grid, new_node.to_string(), visited.clone());
            }
            n
        }
    }
    println!(
        "{:?}",
        find_distinct_path(&parse_input(&input), String::from("start"), vec![])
    ) // 3576
}

pub fn part2(input: String) {
    fn find_distinct_path(
        grid: &Grid,
        start: String,
        mut visited: Vec<String>,
        multi_visit_done: bool,
    ) -> u32 {
        if start == "end" {
            1
        } else {
            let entry = grid.get(&start).unwrap();
            if visited.clone().into_iter().filter(|y| *y == start).count() > 2
                && start.to_lowercase() == *start
                && multi_visit_done
            {
                0
            } else {
                visited.append(&mut vec![start.to_string()]);
                let new_multi_visit_done = multi_visit_done
                    || (start.to_lowercase() == start
                        && visited.clone().into_iter().filter(|y| *y == start).count() == 2);
                let mut n = 0;
                for new_node in entry.clone().into_iter().filter(|x| {
                    x.to_lowercase() != "start"
                        && (x.to_uppercase() == *x
                            || (x.to_lowercase() == *x
                                && (!new_multi_visit_done
                                    || visited.clone().into_iter().filter(|y| y == x).count() < 1)))
                }) {
                    n += find_distinct_path(
                        grid,
                        new_node.to_string(),
                        visited.clone(),
                        new_multi_visit_done,
                    );
                }
                n
            }
        }
    }
    println!(
        "{:?}",
        find_distinct_path(&parse_input(&input), String::from("start"), vec![], false)
    ) // 84271
}
