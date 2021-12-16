use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use bitvec::prelude::*;

enum Packet {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(usize),
    Greater(Vec<Packet>),
    Lesser(Vec<Packet>),
    Equal(Vec<Packet>),
}

fn parse_packet(bv: &BitSlice<Msb0, u8>) -> (Packet, usize) {
    let type_id = bv[3..6].load_be::<u32>();
    let next_start;
    let packet;
    match type_id {
        4 => {
            let mut idx = 6;
            let mut number_bv: BitVec<Msb0, u8> = BitVec::new();
            loop {
                let chunk = &bv[idx..idx + 5];
                number_bv.extend_from_bitslice(&chunk[1..]);
                idx += 5;
                if chunk[0] == false {
                    break;
                }
            }
            let number = number_bv.load_be::<usize>();
            next_start = idx;
            packet = Packet::Literal(number);
        }
        _ => {
            let mut idx = 6;
            let length_type_id = bv[idx];
            idx += 1;
            let mut children = Vec::new();
            if length_type_id {
                // 1
                let window = &bv[idx..idx + 11];
                idx += 11;
                let number_subpackets = window.load_be::<usize>();
                for _ in 0..number_subpackets {
                    let (packet, offset) = parse_packet(&bv[idx..]);
                    children.push(packet);
                    idx += offset;
                }
                next_start = idx;
            } else {
                // 0
                let window = &bv[idx..idx + 15];
                let total_subpacket_length = window.load_be::<usize>();
                idx += 15;
                next_start = idx + total_subpacket_length;
                while idx < next_start {
                    let (packet, offset) = parse_packet(&bv[idx..]);
                    children.push(packet);
                    idx += offset;
                }
            }
            packet = match type_id {
                0 => Packet::Sum(children),
                1 => Packet::Product(children),
                2 => Packet::Minimum(children),
                3 => Packet::Maximum(children),
                5 => Packet::Greater(children),
                6 => Packet::Lesser(children),
                7 => Packet::Equal(children),
                _ => unreachable!(),
            }
        }
    }
    (packet, next_start)
}

impl Packet {
    fn eval(&self) -> usize {
        match self {
            Self::Sum(children) => children.iter().map(Self::eval).sum(),
            Self::Product(children) => children
                .iter()
                .map(Self::eval)
                .reduce(|acc, el| acc * el)
                .unwrap(),
            Self::Minimum(children) => children.iter().map(Self::eval).min().unwrap(),
            Self::Maximum(children) => children.iter().map(Self::eval).max().unwrap(),
            Self::Literal(value) => *value,
            Self::Greater(children) => {
                let first = children[0].eval();
                let second = children[1].eval();
                (first > second) as usize
            }
            Self::Lesser(children) => {
                let first = children[0].eval();
                let second = children[1].eval();
                (first < second) as usize
            }
            Self::Equal(children) => {
                let first = children[0].eval();
                let second = children[1].eval();
                (first == second) as usize
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut bv: BitVec<Msb0, u8> = BitVec::new();

    let line = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap();
    let mut chars = line.chars();

    while let Some(first) = chars.next() {
        let second = chars.next().unwrap();
        let mut string = String::new();
        string.push(first);
        string.push(second);
        let byte = u8::from_str_radix(&string, 16).unwrap();
        bv.extend(byte.view_bits::<Msb0>());
    }

    let (packet, _) = parse_packet(&bv);
    let res = packet.eval();

    println!("Result = {}", res);
    Ok(())
}
