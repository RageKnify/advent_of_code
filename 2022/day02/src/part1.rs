use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res = 0;

    for line in input.lines().map(Result::unwrap) {
        let (elf, me) = line.split_once(" ").unwrap();
        match me {
            "X" => res += 1,
            "Y" => res += 2,
            "Z" => res += 3,
            _ => unreachable!(),
        }
        match (elf, me) {
            ("A", "X") => res += 3,
            ("A", "Y") => res += 6,
            ("A", "Z") => (),
            ("B", "X") => (),
            ("B", "Y") => res += 3,
            ("B", "Z") => res += 6,
            ("C", "X") => res += 6,
            ("C", "Y") => (),
            ("C", "Z") => res += 3,
            _ => unreachable!(),
        };
    }

    println!("Result = {}", res);
    Ok(())
}
