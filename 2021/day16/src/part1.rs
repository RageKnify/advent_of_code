use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use bitvec::prelude::*;

fn parse_packet(bv: &BitSlice<Msb0, u8>) -> (u32, usize) {
    let mut version = bv[..3].load_be::<u32>();
    let type_id = bv[3..6].load_be::<u32>();
    let next_start;
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
            let _number = number_bv.load_be::<usize>();
            next_start = idx;
        }
        _ => {
            let mut idx = 6;
            let length_type_id = bv[idx];
            idx += 1;
            if length_type_id {
                // 1
                let window = &bv[idx..idx + 11];
                idx += 11;
                let number_subpackets = window.load_be::<usize>();
                let mut versions = 0;
                for _ in 0..number_subpackets {
                    let (version, offset) = parse_packet(&bv[idx..]);
                    versions += version;
                    idx += offset;
                }
                version += versions;
                next_start = idx;
            } else {
                // 0
                let window = &bv[idx..idx + 15];
                let total_subpacket_length = window.load_be::<usize>();
                idx += 15;
                next_start = idx + total_subpacket_length;
                let mut versions = 0;
                while idx < next_start {
                    let (version, offset) = parse_packet(&bv[idx..]);
                    versions += version;
                    idx += offset;
                }
                version += versions;
            }
        }
    }

    (version, next_start)
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

    let mut res = 0;
    let mut idx = 0;
    while (idx + 11) < bv.len() {
        let (version, offset) = parse_packet(&bv[idx..]);
        res += version;
        idx += offset;
    }

    println!("Result = {}", res);
    Ok(())
}
