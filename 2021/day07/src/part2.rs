use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn triangle_n(n: usize) -> usize {
    n * (n + 1) / 2
}

fn main() -> std::io::Result<()> {
    let positions: Vec<_> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();

    let start: usize = *positions.iter().min().unwrap();
    let end: usize = *positions.iter().max().unwrap();
    let res: usize = (start..end)
        .map(|m| {
            positions
                .iter()
                .map(|&x| triangle_n(if x < m { m - x } else { x - m }))
                .sum()
        })
        .min()
        .unwrap();

    println!("Result = {}", res);
    Ok(())
}
