enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_line(line: &str) -> Direction {
    let (direction, distance) = line.split_once(' ').unwrap();
    let distance = distance.parse::<i32>().unwrap();

    if direction == "forward" {
        Direction::Forward(distance)
    } else if direction == "down" {
        Direction::Down(distance)
    } else {
        Direction::Up(distance)
    }
}

pub fn part1(input: String) {
    let parsed_lines = input.lines().map(parse_line);
	let mut depth = 0;
	let mut horizontal = 0;
	for line in parsed_lines {
		match line {
            Direction::Forward(distance) => horizontal += distance,
            Direction::Down(distance) => depth += distance,
            Direction::Up(distance) => depth -= distance,
        }
	}
    println!(
        "{}",
        depth * horizontal
    );
}

pub fn part2(input: String) {
    let parsed_lines = input.lines().map(parse_line);
	let mut depth = 0;
	let mut horizontal = 0;
	let mut aim = 0;
	for line in parsed_lines {
		match line {
            Direction::Forward(distance) => {
				horizontal += distance;
				depth += aim * distance;
			},
            Direction::Down(distance) => aim += distance,
            Direction::Up(distance) => aim -= distance,
        }
	}
    println!(
        "{}",
        depth * horizontal
    );
}
