use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut lines = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap);

    let mut dots = HashSet::new();
    loop {
        if let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            let (x, y) = line.split_once(',').unwrap();
            let (x, y) = (usize::from_str(x).unwrap(), usize::from_str(y).unwrap());
            dots.insert((x, y));
        } else {
            break;
        }
    }

    while let Some(line) = lines.next() {
        let mut it = line.split(' ');
        it.next();
        it.next();
        let (axis, idx) = it.next().unwrap().split_once('=').unwrap();
        let idx = usize::from_str(idx).unwrap();
        println!("{}={}", axis, idx);
        dots = dots
            .drain()
            .map(|e| {
                if axis == "x" {
                    if e.0 > idx {
                        let delta = e.0 - idx;
                        (e.0 - (delta * 2), e.1)
                    } else {
                        e
                    }
                } else {
                    if e.1 > idx {
                        let delta = e.1 - idx;
                        (e.0, e.1 - (delta * 2))
                    } else {
                        e
                    }
                }
            })
            .collect();
        break;
    }

    let res = dots.len();

    println!("Result = {}", res);
    Ok(())
}
