use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn count_basins(hm: &[Vec<u8>]) -> usize {
    let max_x = hm.len();
    let max_y = hm[0].len();
    let mut sizes = Vec::new();
    let mut basin_starts = Vec::new();
    let mut visited_points = HashSet::new();
    basin_starts.push((0, 0));
    while let Some(start) = basin_starts.pop() {
        if visited_points.contains(&start) {
            continue;
        }
        let mut basin_queue = Vec::new();
        let mut basin_set = HashSet::new();
        basin_queue.push(start);
        while let Some(point) = basin_queue.pop() {
            let (x, y) = point;
            if visited_points.contains(&point) {
                continue;
            }
            visited_points.insert(point);
            if hm[x][y] < 9 {
                basin_set.insert(point);
            }
            if x > 0 {
                if hm[x - 1][y] < 9 {
                    if !basin_set.contains(&(x - 1, y)) {
                        basin_queue.push((x - 1, y));
                    }
                } else if x > 1 && hm[x - 2][y] < 9 {
                    basin_starts.push((x - 2, y));
                }
            }
            if x < max_x - 1 {
                if hm[x + 1][y] < 9 {
                    if !basin_set.contains(&(x + 1, y)) {
                        basin_queue.push((x + 1, y));
                    }
                } else if x < max_x - 2 && hm[x + 2][y] < 9 {
                    basin_starts.push((x + 2, y));
                }
            }
            if y > 0 {
                if hm[x][y - 1] < 9 {
                    if !basin_set.contains(&(x, y - 1)) {
                        basin_queue.push((x, y - 1));
                    }
                } else if y > 1 && hm[x][y - 2] < 9 {
                    basin_starts.push((x, y - 2));
                }
            }
            if y < max_y - 1 {
                if hm[x][y + 1] < 9 {
                    if !basin_set.contains(&(x, y + 1)) {
                        basin_queue.push((x, y + 1));
                    }
                } else if y < max_y - 2 && hm[x][y + 2] < 9 {
                    basin_starts.push((x, y + 2));
                }
            }
        }
        if !basin_set.is_empty() {
            sizes.push(basin_set.len());
        }
    }
    sizes.sort_unstable_by(|a, b| a.cmp(b).reverse());
    sizes
        .iter()
        .take(3)
        .cloned()
        .reduce(|acc, x| acc * x)
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let heightmap: Vec<Vec<_>> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .map(|c| u8::from_str(&c.to_string()))
                .map(Result::unwrap)
                .collect()
        })
        .collect();
    let res = count_basins(&heightmap);

    println!("Result = {}", res);
    Ok(())
}
