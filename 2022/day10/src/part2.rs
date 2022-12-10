use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut x = 1;
    let mut cycle_values = vec![x];

    for line in input.lines().map(Result::unwrap) {
        match line.split_once(' ') {
            None => cycle_values.push(x),
            Some((_, value)) => {
                cycle_values.push(x);
                let value: i64 = value.parse().unwrap();
                x += value;
                cycle_values.push(x);
            }
        }
    }

    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut res = String::with_capacity(HEIGHT * WIDTH);
    let mut idx = 0;
    for _ in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = {
                let curr_x = cycle_values[idx];
                if (curr_x - (x as i64)).abs() <= 1 {
                    '#'
                } else {
                    '.'
                }
            };
            res.push(pixel);
            idx += 1;
        }
        res.push('\n');
    }
    print!("{}", res);
    Ok(())
}
