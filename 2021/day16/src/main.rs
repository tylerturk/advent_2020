#![allow(dead_code, unused_variables, unused_mut, unused_assignments)]

use std::collections::HashMap;
#[macro_use]
extern crate maplit;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    // println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn get_mapping() -> HashMap<char, String> {
    hashmap![
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
    ]
}

fn input_to_binary(hex: String) -> String {
    let mapping = get_mapping();
    let binary: String = hex.chars().map(|c| mapping.get(&c).unwrap().to_string()).collect::<Vec<String>>().join("");
    binary.to_string()
}

fn convert_string_to_binary(binary: &String) -> i32 {
    i32::from_str_radix(&binary, 2).unwrap()
}

fn parse_packet_version_and_type(bin_vec: &Vec<char>, ind: usize) -> (i32, i32, usize) {
    let packet_version = convert_string_to_binary(&bin_vec[ind..ind+3].iter().map(|c| c.to_string()).collect::<String>());
    let packet_type = convert_string_to_binary(&bin_vec[ind+3..ind+6].iter().map(|c| c.to_string()).collect::<String>());
    (packet_version, packet_type, ind + 6)
}

fn parse_number_from_slice(bin_vec: &Vec<char>, mut ind: usize) -> (i32, usize) {
    let mut bin_slice: Vec<char> = Vec::new();
    let start = ind;
    loop {
        let last = if bin_vec[ind] == '0' {
            true
        } else {
            false
        };
        ind += 1;
        bin_slice.append(&mut bin_vec[ind..ind+4].to_vec());
        ind += 4;
        if last {
            break;
        }
    }
    let num = convert_string_to_binary(&bin_slice.iter().map(|c| c.to_string()).collect::<String>());
    (num, ind)
}

#[derive(Clone, Debug)]
struct Packet {
    version: i32,
    p_type: i32,
    length: Option<usize>,
    binary: String,
    subpackets: Vec<Packet>,
    value: i32,
}

impl Packet {
    fn new(bin_vec: &Vec<char>) -> Self {
        // let bin_vec = binary.chars().collect::<Vec<char>>();
        let mut ind = 6;
        let mut length: Option<usize> = None;
        let mut value = 0;
        let mut subpackets: Vec<Packet> = Vec::new();

        let (version, p_type, _) = parse_packet_version_and_type(&bin_vec, 0);
        if p_type == 4 {
            (value, ind) = parse_number_from_slice(&bin_vec, ind);
            length = Some(ind);
            return Self {
                version,
                p_type,
                length: length.into(),
                binary: bin_vec.iter().collect::<String>(),
                value,
                subpackets,
            };
        } else {
            let length_type = bin_vec[ind];
            ind += 1;
            if length_type == '0' {
                let bit_length = &bin_vec[ind..ind+15].iter().collect::<String>();
                let size: usize = convert_string_to_binary(bit_length) as usize;
                ind += 15;
                subpackets.append(&mut Packet::parse_subpackets(&bin_vec[ind..ind+size].to_vec()));
                ind += size;
            } else {
                let num_packets = convert_string_to_binary(&bin_vec[ind..ind+11].iter().collect::<String>());
                ind += 11;
                for _ in 0..num_packets {
                    let packet = Packet::new(&bin_vec[ind..bin_vec.len()].to_vec());
                    subpackets.push(packet.clone());
                    match packet.length {
                        Some(length) => ind += length,
                        _ => {
                            println!("No length: {:#?}", packet);
                            break
                        }
                    };
                }
            }
        }
        // if ind + 11 < bin_vec.len() {
        //     subpackets.append(&mut Packet::parse_subpackets(&bin_vec[ind..bin_vec.len()].to_vec()))
        // }
        Self {
            version,
            p_type,
            length: length.into(),
            binary: bin_vec.iter().collect::<String>(),
            value,
            subpackets,
        }
    }

    fn parse_subpackets(bin_vec: &Vec<char>) -> Vec<Packet> {
        let mut ind: usize = 0;
        let mut packets: Vec<Packet> = Vec::new();
        while ind < bin_vec.len() {
            if ind + 8 > bin_vec.len() {
                break;
            }
            let packet = Packet::new(&bin_vec[ind..bin_vec.len()].to_vec());
            packets.push(packet.clone());
            match packet.length {
                Some(length) => ind += length,
                _ => break,
            };
        }
        packets
    }

    fn sum_values(&self) -> i32 {
        let mut sum = self.value;
        for packet in &self.subpackets {
            sum += packet.sum_values();
        }
        sum
    }

    fn sum_versions(&self) -> i32 {
        let mut sum = self.version;
        for packet in &self.subpackets {
            sum += packet.sum_versions();
        }
        sum
    }
}

fn solve_packet(input: String) -> Packet {
    let binary = input_to_binary(input.clone());
    let bin_vec = binary.chars().collect::<Vec<char>>();
    let packet = Packet::new(&bin_vec);

    println!("---------------------------------");
    println!("Input: {}", input);
    println!("{:#?}", packet);
    println!("Summed values: {}", packet.sum_values());
    println!("Summed versions: {}", packet.sum_versions());
    println!("---------------------------------");
    packet
}

fn solve_part_1(contents: String) -> i32 {

    // let binary = input_to_binary(contents.trim().to_string());
    // solve_packet("EE00D40C823060".to_string());
    // solve_packet("8A004A801A8002F478".to_string());
    solve_packet("620080001611562C8802118E34".to_string());
    // solve_packet("C0015000016115A2E0802F182340".to_string());
    // solve_packet("A0016C880162017C3686B18A3D4780".to_string());


    0
}

fn solve_part_1_old(contents: String) -> i32 {
    let binary = input_to_binary(contents.trim().to_string());
    let bin_vec = binary.chars().collect::<Vec<char>>();
    println!("{}", binary);
    // let outer = convert_string_to_binary(&bin_vec[0..3].iter().collect::<String>());
    // println!("Parsing packet type: {}", outer);
    let mut ind: usize = 0;
    let mut packet_version: i32;
    let mut packet_type: i32;
    let mut total: i32 = 0;
    let mut version_numbers: i32 = 0;
    while ind < bin_vec.len() {
        if ind + 6 > bin_vec.len() {
            break;
        }
        (packet_version, packet_type, ind) = parse_packet_version_and_type(&bin_vec, ind);
        println!("Parsing packet version {} packet type: {}", packet_version, packet_type);
        version_numbers += packet_version;
        ind += 3;
        if packet_type == 4 {
            let (num, inc) = parse_number_from_slice(&bin_vec, ind);
            ind = inc;
            println!("Adding num: {}", num);
            total += num;
            println!("{:?}", &bin_vec[ind..bin_vec.len()].iter().collect::<String>());
        } else {
            let length_type = bin_vec[ind];
            println!("Detected length type: {}", length_type);
            ind += 1;
            if length_type == '0' {
                let depth = ind + convert_string_to_binary(&bin_vec[ind..ind+15].iter().collect::<String>()) as usize + 15;
                ind += 15;
                while ind < depth {
                    (packet_version, packet_type, ind) = parse_packet_version_and_type(&bin_vec, ind);
                    version_numbers += packet_version;
                    if packet_type != 4 {
                        panic!("Oh no, my code, it's broken {}", packet_type);
                    }
                    let (num, inc) = parse_number_from_slice(&bin_vec, ind);
                    ind = inc;
                }
            } else {
                let num_packets = convert_string_to_binary(&bin_vec[ind..ind+11].iter().collect::<String>());
                println!("Expecting {} packets.", num_packets);
                ind += 11;
                for _ in 0..num_packets {
                    (packet_version, packet_type, ind) = parse_packet_version_and_type(&bin_vec, ind);
                    version_numbers += packet_version;
                    let (num, inc) = parse_number_from_slice(&bin_vec, ind);
                    ind = inc;
                    println!("Adding num: {}", num);
                    total += num;
                    println!("{:?}", &bin_vec[ind..bin_vec.len()].iter().collect::<String>());
                }
            }
        }
    }
    println!("Total: {}", total);
    version_numbers
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        // assert_eq!(solve_part_1("D2FE28".to_string()), 2021);
        // assert_eq!(solve_part_1("EE00D40C823060".to_string()), 16);
        // solve_packet("D2FE28".to_string());
        // solve_packet("38006F45291200".to_string());

        assert_eq!(solve_packet("8A004A801A8002F478".to_string()).sum_versions(), 16);
        assert_eq!(solve_packet("620080001611562C8802118E34".to_string()).sum_versions(), 12);
        assert_eq!(solve_packet("C0015000016115A2E0802F182340".to_string()).sum_versions(), 23);
        assert_eq!(solve_packet("A0016C880162017C3686B18A3D4780".to_string()).sum_versions(), 31);
    }

    // #[test]
    // fn part_1() {
    //     assert_eq!(solve_part_1(aoc::sample()), 0);
    // }

    #[test]
    fn part_2() {
        // assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
