use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap},
    convert::TryInto,
    usize, vec,
};

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        id: usize,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Clone)]
struct Input {
    data: String,
    index: usize,
}

impl Input {
    fn take(&mut self, amount: usize) -> &str {
        let result = &self.data[self.index..self.index + amount];
        self.index += amount;
        result
    }
    fn take_usize(&mut self, amount: usize) -> usize {
        usize::from_str_radix(self.take(amount), 2).unwrap()
    }

    fn skip(&mut self, amount: usize) -> () {
        self.index += amount
    }

    fn is_finished(&self) -> bool {
        // println!("is finished ? {:?} >= {:?}", self.index, self.data.len());
        self.index >= self.data.len() - 6
    }
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_input(input: &String) -> Option<Packet> {
    let mut binary_input = Input {
        data: input
            .chars()
            .map(to_binary)
            .fold("".to_string(), |mut i, j| {
                i.push_str(&*j);
                i
            }),
        index: 0,
    };

    fn parse_packet(binary_input: &mut Input) -> Option<Packet> {
        // println!("{:?}: {:?}", binary_input.index, binary_input.data);
        let mut result = None;
        // while !binary_input.is_finished() {
        let version = binary_input.take_usize(3);
        let id = binary_input.take_usize(3);
        match id {
            4 => {
                // println!("Found literal");
                let mut to_parse = "".to_string();
                while binary_input.take(1) == "1" {
                    to_parse.push_str(binary_input.take(4));
                }
                to_parse.push_str(binary_input.take(4));
                result = Some(Packet::Literal {
                    version,
                    value: usize::from_str_radix(&to_parse, 2).unwrap(),
                })
            }
            _ => match binary_input.take(1) {
                "0" => {
                    // println!("Found 15 bits, {:?}", binary_input.index);
                    // next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet
                    let mut sub_packets = vec![];
                    let length_to_read = binary_input.take_usize(15);
                    let start_index = binary_input.index;
                    while binary_input.index < start_index + length_to_read {
                        sub_packets.push(parse_packet(binary_input).unwrap());
                    }
                    result = Some(Packet::Operator {
                        version,
                        id,
                        sub_packets,
                    })
                }
                _ => {
                    // println!("Found 11 bits");
                    // next 11 bits are a number that represents the number of sub-packets immediately contained by this packet
                    result = Some(Packet::Operator {
                        version,
                        id,
                        sub_packets: (0..binary_input.take_usize(11)).into_iter().fold(
                            vec![],
                            |mut acc, _| {
                                acc.push(parse_packet(binary_input).unwrap());
                                // println!("{:?}", binary_input.index);
                                acc
                            },
                        ),
                    })
                }
            },
        }
        // }
        result
    }

    parse_packet(&mut binary_input)
}

pub fn part1(input: String) {
    fn calculate_version_sums(packet: &Packet) -> usize {
        match packet {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => {
                *version
                    + sub_packets
                        .iter()
                        .map(calculate_version_sums)
                        .sum::<usize>()
            }
        }
    }
    match parse_input(&input) {
        Some(packet) => println!("{:?}", calculate_version_sums(&packet)),
        None => {}
    }
}

pub fn part2(input: String) {
    // match parse_input(&input) {
    //     Some(packet) => println!("{:?}", calculate_version_sums(&packet)),
    //     None => {}
    // }
}
