use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn simulate(fish: Vec<u8>) -> Vec<u8> {
    let mut new = 0;
    let mut advanced_fish: Vec<u8> = fish
        .iter()
        .map(|&i| {
            if i == 0u8 {
                new += 1;
                6
            } else {
                i - 1
            }
        })
        .collect();
    advanced_fish.reserve(new);
    advanced_fish.extend_from_slice(&vec![8; new]);
    advanced_fish
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut fish: Vec<u8> = input
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect();

    for _ in 0..80 {
        fish = simulate(fish);
    }

    let res = fish.len();

    println!("Result = {}", res);
    Ok(())
}
