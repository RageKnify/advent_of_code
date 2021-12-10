use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn points_line(line: &str) -> Option<usize> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' => {
                stack.push(')');
            }
            '[' => {
                stack.push(']');
            }
            '{' => {
                stack.push('}');
            }
            '<' => {
                stack.push('>');
            }
            ')' | ']' | '}' | '>' => {
                let expected = stack.pop();
                if expected != Some(c) {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }
    stack
        .iter()
        .rev()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        })
        .reduce(|acc, x| acc * 5 + x)
}

fn main() -> std::io::Result<()> {
    let mut res: Vec<_> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|l| points_line(&l))
        .flatten()
        .collect();

    res.sort_unstable();
    let mut res = VecDeque::from(res);

    while res.len() != 1 {
        res.pop_front();
        res.pop_back();
    }

    println!("Result = {}", res[0]);

    Ok(())
}
