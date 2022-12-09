use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const ABOVE_DIAGONAL_DISTANCE: f32 = 1.5f32;
const SNAKE_LENGTH: usize = 10;
type Snake = [(i32, i32); SNAKE_LENGTH];

fn xy_distance(head: (i32, i32), tail: (i32, i32)) -> f32 {
    let head = (head.0 as f32, head.1 as f32);
    let tail = (tail.0 as f32, tail.1 as f32);

    let x_distance = head.0 - tail.0;
    let y_distance = head.1 - tail.1;

    (x_distance.powi(2) + y_distance.powi(2)).sqrt()
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut snake: Snake = [(0, 0); SNAKE_LENGTH];

    let mut visited = HashSet::new();
    visited.insert(snake[SNAKE_LENGTH-1]);

    for line in input.lines().map(Result::unwrap) {
        let (dir, length) = line.split_once(' ').unwrap();
        let length: usize = length.parse().unwrap();
        advance(&mut snake, dir, length, &mut visited);
    }

    println!("Result = {}", visited.len());
    Ok(())
}

fn advance(snake: &mut Snake, dir: &str, length: usize, visited: &mut HashSet<(i32, i32)>) {
    fn update_dir(head: &mut (i32, i32), dir: &str) {
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
    }
    fn update_diagonal(new: (i32, i32), knot: &mut(i32, i32)) {
        if (new.0 - knot.0).abs() > 1 && (new.1 - knot.1).abs() > 1 {
            // mega diagonal, average both to move to center
            knot.0 = (new.0 + knot.0)/2;
            knot.1 = (new.1 + knot.1)/2;
        } else if (new.0 - knot.0).abs() > 1 {
            // horizontal
            knot.0 = (new.0 + knot.0)/2;
            knot.1 = new.1;
        } else if (new.1 - knot.1).abs() > 1 {
            // vertical
            knot.1 = (new.1 + knot.1)/2;
            knot.0 = new.0;
        }
    }
    for _ in 0..length {
        update_dir(&mut snake[0], dir);
        let mut new = snake[0];
        for knot in &mut snake[1..] {
            let distance = xy_distance(new, *knot);
            if distance > 2f32 {
                // diagonal +
                update_diagonal(new, knot);
            } else if distance > ABOVE_DIAGONAL_DISTANCE {
                // 2 apart, averaging fixes
                knot.0 = (new.0 + knot.0)/2;
                knot.1 = (new.1 + knot.1)/2;
            } else {
                // diagonal, do nothing
            }
            new = *knot;
        }
        visited.insert(snake[SNAKE_LENGTH-1]);
    }
}
