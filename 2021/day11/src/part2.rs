use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn step(octupi: &mut Vec<Vec<u32>>) -> usize {
    for x in 0..octupi.len() {
        for y in 0..octupi[0].len() {
            octupi[x][y] += 1;
        }
    }
    let mut flashes = 0;
    let max_x = octupi.len() - 1;
    let max_y = octupi[0].len() - 1;
    loop {
        let mut flashed = false;
        for x in 0..octupi.len() {
            for y in 0..octupi[0].len() {
                if octupi[x][y] >= 10 && octupi[x][y] < 23 {
                    octupi[x][y] = 23;
                    flashed = true;
                    flashes += 1;
                    if x > 0 {
                        octupi[x - 1][y] += 1;
                    }
                    if x < max_x {
                        octupi[x + 1][y] += 1;
                    }
                    if y > 0 {
                        octupi[x][y - 1] += 1;
                    }
                    if y < max_y {
                        octupi[x][y + 1] += 1;
                    }
                    if x > 0 && y > 0 {
                        octupi[x - 1][y - 1] += 1;
                    }
                    if x > 0 && y < max_y {
                        octupi[x - 1][y + 1] += 1;
                    }
                    if x < max_x && y > 0 {
                        octupi[x + 1][y - 1] += 1;
                    }
                    if x < max_x && y < max_y {
                        octupi[x + 1][y + 1] += 1;
                    }
                }
            }
        }
        if !flashed {
            break;
        }
    }
    for x in 0..octupi.len() {
        for y in 0..octupi[0].len() {
            if octupi[x][y] > 9 {
                octupi[x][y] = 0;
            }
        }
    }
    flashes
}

fn main() -> std::io::Result<()> {
    let mut octupi: Vec<Vec<u32>> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let res = (0..)
        .map(|i| (i, step(&mut octupi)))
        .filter(|t| t.1 == 100)
        .map(|t| t.0 + 1)
        .next()
        .unwrap();
    println!("Result = {}", res);

    Ok(())
}
