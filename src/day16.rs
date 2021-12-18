use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap},
    convert::TryInto,
    usize, vec,
};
enum Packet {
    Literal {
        version: u32,
        value: u64,
    },
    Operator {
        version: u32,
        tag: u32,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Clone)]
struct BinaryInput {
    data: String,
    index: usize,
}

impl BinaryInput {
    fn take(&mut self, amount: usize) -> &str {
        let result = &self.data[self.index..self.index + amount];
        self.index += amount;
        result
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

fn to_number(n: &str) -> usize {
    match n {
        "001" => 1,
        "010" => 2,
        "011" => 3,
        "100" => 4,
        "101" => 5,
        "110" => 6,
        "111" => 7,
        _ => 0,
    }
}

// fn parse_binary_input(binary_input: &mut BinaryInput) -> BinaryInput {
    // // println!("parsing {:?}", binary_input.data);
    // let mut result = 0;
    // while !binary_input.is_finished() {
    //     let version = usize::from_str_radix(binary_input.take(3), 2).unwrap();
    //     let id = usize::from_str_radix(binary_input.take(3), 2).unwrap();
    //     println!("version {:?}, id {:?}", version, id);
    //     result = version;
    //     match id {
    //         4 => {
    //             let mut to_parse = "".to_string();
    //             println!(" binary input {:?}", binary_input.data);
    //             while binary_input.take(1) == "1" {
    //                 to_parse.push_str(binary_input.take(4));
    //             }
    //             to_parse.push_str(binary_input.take(4));
    //             println!(
    //                 "{:?} {:?}",
    //                 to_parse,
    //                 usize::from_str_radix(&to_parse, 2).unwrap()
    //             );
    //         }
    //         _ => {
    //             match binary_input.take(1) {
    //                 "0" => {
    //                     println!("0");
    //                     // total length of subpackets in next 15 bits
    //                     let length = usize::from_str_radix(&binary_input.take(15), 2).unwrap();
    //                     let mut result_binary_input = parse_binary_input(&mut BinaryInput {
    //                         data: binary_input.take(length).to_string(),
    //                         index: 0,
    //                     });
    //                 }
    //                 _ => {
    //                     println!("1");
    //                     // number of subpackets contained in next 11 bits
    //                     let amount = usize::from_str_radix(&binary_input.take(11), 2).unwrap();
    //                     for _ in 0..amount {
    //                         let temp_result = parse_binary_input(binary_input);
    //                         // println!("1: {:?}", temp_result);
    //                         result += temp_result;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("result: {:?}", result);
    // binary_input.clone()
// }

// 8A004A801A8002F478 represents an operator packet (version 4) which contains an operator packet (version 1) which contains an operator packet (version 5) which contains a literal value (version 6); this packet has a version sum of 16.

fn parse_input(input: &String) -> BinaryInput {
    let mut binary_input = BinaryInput {
        data: input
            .chars()
            .map(to_binary)
            .fold("".to_string(), |mut i, j| {
                i.push_str(&*j);
                i
            }),
        index: 0,
    };
    // println!("{:?}", binary_input.data);
    // while !binary_input.is_finished() {
    //     let version = usize::from_str_radix(binary_input.take(3), 2).unwrap();
    //     let id = usize::from_str_radix(binary_input.take(3), 2).unwrap();
    //     println!("{:?} {:?}", version, id);
    //     match id {
    //         4 => {
    //             let mut to_parse = "".to_string();
    //             while binary_input.take(1) == "1" {
    //                 to_parse.push_str(binary_input.take(4));
    //             }
    //             to_parse.push_str(binary_input.take(4));
    //             println!(
    //                 "{:?} {:?}",
    //                 to_parse,
    //                 usize::from_str_radix(&to_parse, 2).unwrap()
    //             );
    //         }
    //         _ => {
    //             match binary_input.take(1) {
    //                 "0" => {
    //                     // total length of subpackets in next 15 bits
    //                     let length = usize::from_str_radix(&binary_input.take(15), 2).unwrap();
    //                     println!(
    //                         "{:?}",
    //                         usize::from_str_radix(binary_input.take(length), 2).unwrap()
    //                     )
    //                 }
    //                 _ => {
    //                     // number of subpackets contained in next 11 bits
    //                 }
    //             }
    //         }
    //     }
    //     result.push(Packet {
    //         version: version,
    //         id: id,
    //     })
    // }
    parse_binary_input(&mut binary_input)
    // binary_input
    //     .iter()
    //     .enumerate()
    //     .into_iter()
    //     .for_each(|(i, c)| println!("{:?}", c));
}

pub fn part1(input: String) {
    println!("{:?}", parse_input(&input).index);
}

pub fn part2(input: String) {}
