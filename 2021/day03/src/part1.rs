use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let lines: Vec<String> = input.lines().map(Result::unwrap).collect();

    let numbers = lines.len();
    let mut gamma = 0;
    let n_bits = lines[0].len();

    for i in 0..n_bits {
        gamma = gamma << 1;
        let zeroes = lines.iter().filter(|s| s.as_bytes()[i] == b'0').count();
        let bit: i32 = if zeroes > numbers / 2 { 0 } else { 1 };
        gamma = gamma | bit;
    }

    let epsilon = (gamma ^ (2 << n_bits - 1) - 1) & ((2 << n_bits - 1) - 1);

    let res = gamma * epsilon;
    println!("Result = {}", res);
    Ok(())
}
