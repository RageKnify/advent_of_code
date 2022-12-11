use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct TopThree {
    a: u32,
    b: u32,
    c: u32,
}

impl TopThree {
    fn update(&mut self, mut new: u32) {
        if new >= self.a {
            std::mem::swap(&mut self.a, &mut new);
        }
        if new >= self.b {
            std::mem::swap(&mut self.b, &mut new);
        }
        if new >= self.c {
            self.c = new;
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut top_three = TopThree { a: 0, b: 0, c: 0 };
    let mut curr = 0;

    for line in input.lines().map(Result::unwrap) {
        match line.parse::<u32>() {
            Ok(n) => {
                curr += n;
            }
            // this is a blank line separating 2 elves
            Err(_) => {
                top_three.update(curr);
                curr = 0;
            }
        }
    }
    // checking the last elf
    top_three.update(curr);

    println!("Result = {}", top_three.a + top_three.b + top_three.c);
    Ok(())
}
