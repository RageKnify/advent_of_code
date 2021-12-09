use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn count_low(hm: &[Vec<u8>]) -> usize {
    let mut res = 0;
    let max_x = hm.len();
    let max_y = hm[0].len();
    for x in 0..max_x {
        for y in 0..max_y {
            let mut low = true;
            let curr = hm[x][y];
            if x > 0 && hm[x - 1][y] <= curr {
                low = false;
            }
            if x < max_x - 1 && hm[x + 1][y] <= curr {
                low = false;
            }
            if y > 0 && hm[x][y - 1] <= curr {
                low = false;
            }
            if y < max_y - 1 && hm[x][y + 1] <= curr {
                low = false;
            }
            if low {
                res += 1 + curr as usize;
            }
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let heightmap: Vec<Vec<_>> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .map(|c| u8::from_str(&c.to_string()))
                .map(Result::unwrap)
                .collect()
        })
        .collect();
    let res = count_low(&heightmap);

    println!("Result = {}", res);
    Ok(())
}
