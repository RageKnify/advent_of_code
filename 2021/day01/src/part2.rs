use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let nums: Vec<_> = file.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let mut previous = nums[0] + nums[1] + nums[2];
    let mut res = 0;
    for i in 1..(nums.len() - 2) {
        let curr = nums[i] + nums[i + 1] + nums[i + 2];
        if curr > previous {
            res += 1;
        }
        previous = curr;
    }

    println!("{}", res);
    Ok(())
}
