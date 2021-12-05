struct Board {
    numbers: Vec<u8>,
    marked: [bool; 25],
}

fn parse_boards(input: String) -> (Vec<u8>, Vec<Board>) {
    let mut split_input = input.trim().split("\n\n");
    let bingo_numbers: Vec<u8> = split_input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let boards: Vec<_> = split_input
        .map(|board| {
            board
                .replace('\n', " ")
                .replace("  ", " ")
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| Board {
            numbers: x,
            marked: [false; 25],
        })
        .collect();
    return (bingo_numbers, boards);
}

fn draw_bingo_number_boards(bingo_number: u8, boards: Vec<Board>) -> Vec<Board> {
    return boards
        .into_iter()
        .map(|board| draw_bingo_number(bingo_number, board))
        .collect();
}

fn draw_bingo_number(bingo_number: u8, mut board: Board) -> Board {
    match board.numbers.iter().position(|&x| x == bingo_number) {
        Some(position) => {
            board.marked[position] = true;
            return board;
        }
        None => board,
    }
}

fn calculate_board_score(board: &Board) -> u32 {
    return board
        .marked
        .to_vec()
        .into_iter()
        .zip(&board.numbers)
        .filter(|(mark, _)| !mark)
        .map(|(_, &n)| n as u32)
        .sum();
}

fn is_winner(board: &Board) -> bool {
    for i in (0..=20).step_by(5) {
        if (0..5).all(|x| board.marked[i + x]) {
            return true;
        }
    }
    for i in (0..5).into_iter() {
        if (0..=20).step_by(5).all(|x| board.marked[i + x]) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: String) {
    fn find_winner(boards: &Vec<Board>) -> Option<&Board> {
        return boards.into_iter().find(|board| is_winner(board));
    }

    let (bingo_numbers, mut boards) = parse_boards(input);

    for bingo_number in bingo_numbers.into_iter() {
        boards = draw_bingo_number_boards(bingo_number, boards);
        match find_winner(&boards) {
            Some(board) => {
                println!("{:?}", calculate_board_score(board) * bingo_number as u32);
                break;
            }
            None => (),
        }
    }
}

pub fn part2(input: String) {
    fn find_only_loser(boards: &Vec<Board>) -> Option<&Board> {
        return boards.into_iter().find(|x| !is_winner(x));
    }
    fn has_one_loser(boards: &Vec<Board>) -> bool {
        return boards.into_iter().filter(|x| !is_winner(x)).count() == 1;
    }

    let (bingo_numbers, boards) = parse_boards(input);

    fn draw_loop(mut bingo_numbers: Vec<u8>, boards: Vec<Board>) -> u32 {
        let (bingo_number, remaining_bingo_numbers) = bingo_numbers.split_at_mut(1);
        let new_boards = draw_bingo_number_boards(*bingo_number.first().unwrap(), boards);
        if new_boards.len() == 1 {
            let board = new_boards.first().unwrap();
            if !is_winner(&board) {
                return draw_loop(remaining_bingo_numbers.to_vec(), new_boards);
            } else {
                return calculate_board_score(board) * *bingo_number.first().unwrap() as u32;
            }
        } else if has_one_loser(&new_boards) {
            let board = find_only_loser(&new_boards).unwrap();
            return draw_loop(
                remaining_bingo_numbers.to_vec(),
                vec![Board {
                    numbers: board.numbers.clone(),
                    marked: board.marked,
                }],
            );
        } else {
            return draw_loop(remaining_bingo_numbers.to_vec(), new_boards);
        }
    }

    println!("{:?}", draw_loop(bingo_numbers, boards));
}
