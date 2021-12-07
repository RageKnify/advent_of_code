use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut positions: Vec<_> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();

    positions.sort();
    let mid_idx = positions.len() / 2;
    let mid = (positions[mid_idx] + positions[mid_idx - 1]) / 2;

    let res: usize = positions
        .iter()
        .map(|&x| if x < mid { mid - x } else { x - mid })
        .sum();

    println!("Result = {}", res);
    Ok(())
}
