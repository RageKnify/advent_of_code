use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res = 0;

    for line in input.lines().map(Result::unwrap) {
        let (elf, outcome) = line.split_once(" ").unwrap();
        let me = match (elf, outcome) {
            ("A", "X") => {
                res += 0;
                "C"
            }
            ("B", "X") => {
                res += 0;
                "A"
            }
            ("C", "X") => {
                res += 0;
                "B"
            }
            ("A", "Y") => {
                res += 3;
                "A"
            }
            ("B", "Y") => {
                res += 3;
                "B"
            }
            ("C", "Y") => {
                res += 3;
                "C"
            }
            ("A", "Z") => {
                res += 6;
                "B"
            }
            ("B", "Z") => {
                res += 6;
                "C"
            }
            ("C", "Z") => {
                res += 6;
                "A"
            }
            _ => unreachable!(),
        };
        match me {
            "A" => res += 1,
            "B" => res += 2,
            "C" => res += 3,
            _ => unreachable!(),
        };
    }

    println!("Result = {}", res);
    Ok(())
}
