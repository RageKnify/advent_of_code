use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut map: Vec<Vec<u8>> = vec![];

    for line in input.lines().map(Result::unwrap) {
        map.push(line.into());
    }

    let side = map.len();

    let mut res = 0;

    for x in 1..(side-1) {
        for y in 1..(side-1) {
            let score = calculate_score(&map, x, y);
            if score > res {
                res = score;
            }
        }
    }

    println!("Result = {}", res);
    Ok(())
}

fn calculate_score(map: &[Vec<u8>], x: usize, y: usize) -> usize {
    let side = map.len();
    let house_height = map[y][x];

    let mut up = 0;
    for y_i in (0..y).rev() {
        up += 1;
        let height = map[y_i][x];
        if height >= house_height {
            break;
        }
    }

    let mut left = 0;
    for x_i in (0..x).rev() {
        left += 1;
        let height = map[y][x_i];
        if height >= house_height {
            break;
        }
    }

    let mut down = 0;
    for y_i in (y+1)..side {
        down += 1;
        let height = map[y_i][x];
        if height >= house_height {
            break;
        }
    }

    let mut right = 0;
    for x_i in (x+1)..side {
        right += 1;
        let height = map[y][x_i];
        if height >= house_height {
            break;
        }
    }

    up * left * down * right
}
