use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut x = 1;
    let mut cycle_values = vec![x];

    for line in input.lines().map(Result::unwrap) {
        match line.split_once(' ') {
            None => cycle_values.push(x),
            Some((_, value)) => {
                cycle_values.push(x);
                let value: i64 = value.parse().unwrap();
                x += value;
                cycle_values.push(x);
            }
        }
    }

    const INDEXES: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let res: i64 = INDEXES
        .iter()
        .map(|idx| cycle_values[idx - 1] * (*idx as i64))
        .sum();
    println!("Result = {}", res);
    Ok(())
}
