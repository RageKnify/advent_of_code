use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut max = 0;
    let mut curr = 0;

    for line in input.lines().map(Result::unwrap) {
        match line.parse::<u32>() {
            Ok(n) => {
                curr += n;
            }
            Err(_) => {
                // this is a blank line separating 2 elves
                if curr > max {
                    max = curr;
                }
                curr = 0;
            }
        }
    }
    // checking the last elf
    if curr > max {
        max = curr;
    }

    println!("Result = {}", max);
    Ok(())
}
