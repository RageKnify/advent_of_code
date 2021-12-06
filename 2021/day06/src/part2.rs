use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn simulate(f: u8, days: usize, lookup: &mut HashMap<(u8, usize), usize>) -> usize {
    match lookup.get(&(f, days)) {
        Some(v) => *v,
        None => {
            let mut res = 1;
            if days > f.into() {
                let mut rem = days as i64;
                rem -= f as i64;
                while rem > 0 {
                    res += simulate(9, rem as usize, lookup);
                    rem -= 7;
                }
            }
            lookup.insert((f, days), res);
            res
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    const DAYS: usize = 256;

    let mut lookup = HashMap::new();

    let res: usize = input
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .map(|f| simulate(f, DAYS, &mut lookup))
        .sum();

    println!("Result = {}", res);
    Ok(())
}
