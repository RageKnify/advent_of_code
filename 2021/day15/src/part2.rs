use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type RiskMap = Vec<Vec<u32>>;

fn explore(lines: &RiskMap) -> u32 {
    let n = lines.len();
    let start = (0, 0);
    let end = (5 * n - 1, 5 * n - 1);
    let mut queue = HashMap::new();
    let mut total_risk_levels = HashMap::new();
    queue.insert(start, 0);
    total_risk_levels.insert(start, 0);
    while !total_risk_levels.contains_key(&end) {
        let (&(x, y), prev_cost) = queue
            .iter()
            .reduce(|best, curr| if best.1 < curr.1 { best } else { curr })
            .unwrap();
        let cost = prev_cost + {
            match (x / n, y / n) {
                (0, 0) => lines[x][y],
                (x_inc, y_inc) => {
                    let mut v = lines[x % n][y % n] + x_inc as u32 + y_inc as u32;
                    while v > 9 {
                        v -= 9;
                    }
                    v
                }
            }
        };
        let cost = *total_risk_levels.entry((x, y)).or_insert(cost);
        [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|delta| (x as i32 + delta.0, y as i32 + delta.1))
            .filter(|p| p.0 >= 0 && p.0 < (n * 5) as i32 && p.1 >= 0 && p.1 < (n * 5) as i32)
            .map(|p| (p.0 as usize, p.1 as usize))
            .for_each(|p| {
                if !total_risk_levels.contains_key(&p) {
                    queue
                        .entry(p)
                        .and_modify(|v| {
                            *v = (*v).min(cost);
                        })
                        .or_insert(cost);
                }
            });
        queue.remove(&(x, y));
    }
    total_risk_levels[&end]
}

fn main() -> std::io::Result<()> {
    let lines: RiskMap = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let res = explore(&lines);

    println!("Result = {}", res);
    Ok(())
}
