use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut nums = file.lines().map(|l| l.parse::<u32>().unwrap());

    let mut prev = nums.next().unwrap();

    let mut res = 0;

    for n in nums {
        if n > prev {
            res += 1;
        }
        prev = n;
    }

    println!("{}", res);
    Ok(())
}
