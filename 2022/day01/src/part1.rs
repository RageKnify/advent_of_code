#![feature(const_for)]
#![feature(const_slice_index)]
#![feature(const_option)]

const fn parse_u32(line: &[u8]) -> Option<u32> {
    if line.is_empty() {
        return None;
    }
    let mut res: u32 = 0;
    let mut idx = 0;
    let len = line.len();
    while idx < len {
        let b = line[idx];
        res *= 10;
        res += (b - b'0') as u32;
        idx += 1;
    }
    Some(res)
}

const fn find_nl(bytes: &[u8]) -> usize {
    let mut idx = 0;
    loop {
        if bytes[idx] == b'\n' {
            return idx;
        }
        idx += 1;
    }
}

const fn calculate(input: &str) -> u32 {
    let mut max = 0;
    let mut curr = 0;

    let input_bytes = input.as_bytes();

    let mut start = 0;
    let mut end: usize = find_nl(input_bytes);
    while end < input_bytes.len() {
        let line = input_bytes.get(start..end).unwrap();
        match parse_u32(line) {
            Some(n) => {
                curr += n;
            }
            None => {
                // this is a blank line separating 2 elves
                if curr > max {
                    max = curr;
                }
                curr = 0;
            }
        }
        start = end + 1;
        let rest = input_bytes.get(end + 1..).unwrap();
        if rest.is_empty() {
            break;
        }
        let next_ln = find_nl(rest);
        end += 1 + next_ln;
    }
    // checking the last elf
    if curr > max {
        max = curr;
    }

    max
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    const RES: u32 = calculate(INPUT);
    println!("Result = {}", RES);
}
