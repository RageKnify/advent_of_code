use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res = 0;

    for line in input.lines().map(Result::unwrap) {
        let (first, second) = line.split_once(',').unwrap();
        let (a, b) = first.split_once('-').unwrap();
        let (a, b) = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
        let (c, d) = second.split_once('-').unwrap();
        let (c, d) = (c.parse::<u32>().unwrap(), d.parse::<u32>().unwrap());
        if (a <= c && d <= b) || (c <= a && b <= d) {
            res += 1;
        }
    }

    println!("Result = {}", res);
    Ok(())
}
