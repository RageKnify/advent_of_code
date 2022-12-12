use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn adjacents(map: &[u8], x: usize, y: usize, x_n: usize, y_n: usize) -> Vec<usize> {
    let curr = map[x + y * x_n];
    let mut res = vec![];
    if x < x_n - 1 && map[x + 1 + y * x_n] - 1 <= curr {
        res.push(x + 1 + y * x_n);
    }
    if x > 0 && map[x - 1 + y * x_n] - 1 <= curr {
        res.push(x - 1 + y * x_n);
    }
    if y < y_n - 1 && map[x + (y + 1) * x_n] - 1 <= curr {
        res.push(x + (y + 1) * x_n);
    }
    if y > 0 && map[x + (y - 1) * x_n] - 1 <= curr {
        res.push(x + (y - 1) * x_n);
    }
    res
}

fn solve(map: &mut Vec<u8>, x_n: usize) -> Option<usize> {
    let start = {
        let start_idx = map.iter().position(|&c| c == b'S').unwrap();
        map[start_idx] = b'a';
        let x = start_idx % x_n;
        let y = start_idx / x_n;
        State { cost: 0, x, y }
    };
    let end_idx = map.iter().position(|&c| c == b'E').unwrap();
    map[end_idx] = b'z';
    let end = (end_idx % x_n, end_idx / x_n);
    let y_n = map.len() / x_n;

    let mut dist: Vec<_> = (0..x_n * y_n).map(|_| usize::MAX).collect();
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(start);

    while let Some(State { cost, x, y }) = heap.pop() {
        if x == end.0 && y == end.1 {
            return Some(cost);
        }

        let idx = x + y * x_n;
        if cost > dist[idx] {
            continue;
        }

        for &next_idx in &adjacents(map, x, y, x_n, y_n) {
            let (next_x, next_y) = (next_idx % x_n, next_idx / x_n);
            let next_cost = cost + 1;
            if next_cost < dist[next_idx] {
                let next = State {
                    cost: next_cost,
                    x: next_x,
                    y: next_y,
                };
                heap.push(next);
                dist[next_idx] = next_cost;
            }
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let mut lines = input.lines().map(Result::unwrap);

    let first_line = lines.next().unwrap();
    let mut map = first_line.into_bytes();
    let x_n = map.len();
    lines.for_each(|s| map.extend_from_slice(s.as_bytes()));

    let res = solve(&mut map, x_n).unwrap();

    println!("Result = {}", res);
    Ok(())
}
