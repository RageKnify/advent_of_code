use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn xy_distance(head: (i32, i32), tail: (i32, i32)) -> f32 {
    let head = (head.0 as f32, head.1 as f32);
    let tail = (tail.0 as f32, tail.1 as f32);

    let x_distance = head.0 - tail.0;
    let y_distance = head.1 - tail.1;

    (x_distance.powi(2) + y_distance.powi(2)).sqrt()
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.lines().map(Result::unwrap) {
        let (dir, length) = line.split_once(' ').unwrap();
        let length: usize = length.parse().unwrap();
        const MAX_DISTANCE: f32 = 1.5f32;
        for _ in 0..length {
            let previous = head;
            match dir {
                "U" => {
                    head.1 += 1;
                }
                "D" => {
                    head.1 -= 1;
                }
                "L" => {
                    head.0 -= 1;
                }
                "R" => {
                    head.0 += 1;
                }
                _ => unreachable!(),
            }
            let distance = xy_distance(head, tail);
            if distance > MAX_DISTANCE {
                tail = previous;
            }
            visited.insert(tail);
        }
    }

    println!("Result = {}", visited.len());
    Ok(())
}
