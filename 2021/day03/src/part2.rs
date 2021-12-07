use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_oxygen(lines: &Vec<String>) -> usize {
    let mut lines = lines.clone();

    let mut i = 0;
    while lines.len() > 1 {
        let zeroes = lines.iter().filter(|s| s.as_bytes()[i] == b'0').count();
        let entries = lines.len();
        lines.retain(|e| {
            let x = if zeroes > entries / 2 { b'0' } else { b'1' };
            e.as_bytes()[i] == x
        });
        i += 1;
    }

    usize::from_str_radix(&lines[0], 2).unwrap()
}

fn find_co2(lines: &Vec<String>) -> usize {
    let mut lines = lines.clone();

    let mut i = 0;
    while lines.len() > 1 {
        let zeroes = lines.iter().filter(|s| s.as_bytes()[i] == b'0').count();
        let entries = lines.len();
        lines.retain(|e| {
            let x = if zeroes > entries / 2 { b'1' } else { b'0' };
            e.as_bytes()[i] == x
        });
        i += 1;
    }

    usize::from_str_radix(&lines[0], 2).unwrap()
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let lines: Vec<String> = input.lines().map(Result::unwrap).collect();

    let oxygen = find_oxygen(&lines);
    let co2 = find_co2(&lines);

    let res = oxygen * co2;
    println!("Result = {}", res);
    Ok(())
}
