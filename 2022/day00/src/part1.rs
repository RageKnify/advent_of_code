use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res = 0;

    for line in input.lines().map(Result::unwrap) {
        todo!();
    }

    println!("Result = {}", res);
    Ok(())
}
