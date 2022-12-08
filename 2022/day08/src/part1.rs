use std::collections::HashSet;
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
    let outer = (side + side - 2) * 2;

    let mut inner = HashSet::new();

    for x in 1..(side-1) {
        let mut curr_height = map[x][0];
        for y in 1..(side-1) {
            let height = map[x][y];
            if height > curr_height {
                inner.insert((x, y));
                curr_height = height;
            }
        }
        let mut curr_height = map[x][side-1];
        for y in (1..(side-1)).rev() {
            let height = map[x][y];
            if height > curr_height {
                inner.insert((x, y));
                curr_height = height;
            }
        }
    }
    for y in 1..(side-1) {
        let mut curr_height = map[0][y];
        for x in 1..(side-1) {
            let height = map[x][y];
            if height > curr_height {
                inner.insert((x, y));
                curr_height = height;
            }
        }
        let mut curr_height = map[side-1][y];
        for x in (1..(side-1)).rev() {
            let height = map[x][y];
            if height > curr_height {
                inner.insert((x, y));
                curr_height = height;
            }
        }
    }

    println!("Result = {}", outer+inner.len());
    Ok(())
}
