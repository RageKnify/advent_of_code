use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn points_line(line: &str) -> usize {
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
                    return match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                }
            }
            _ => unreachable!(),
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let res: usize = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|l| points_line(&l))
        .sum();

    println!("Result = {}", res);

    Ok(())
}
