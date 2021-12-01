pub fn part1(input: String) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|window| &window[1] > &window[0])
            .count()
    );
}

pub fn part2(input: String) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(3)
            .map(|window| &window[0] + &window[1] + &window[2])
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|window| &window[1] > &window[0])
            .count()
    );
}
