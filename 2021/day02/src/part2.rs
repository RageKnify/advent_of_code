use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in input.lines().map(Result::unwrap) {
        let (word, n) = line.split_once(" ").unwrap();
        let n: u32 = n.parse().unwrap();
        match word {
            "forward" => {
                x += n;
                y += aim * n
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => unreachable!(),
        }
    }
    let res = x * y;

    println!("Result = {}", res);
    Ok(())
}
