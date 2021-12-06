use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

fn parse_pair(pair: &str) -> (u32, u32) {
    let (x, y) = pair.split_once(',').unwrap();
    (u32::from_str(x).unwrap(), u32::from_str(y).unwrap())
}

impl Line {
    fn from_str(s: String) -> Self {
        let (start, rest) = s.split_once(' ').unwrap();
        let (_, end) = rest.split_once(' ').unwrap();
        let start = parse_pair(start);
        let end = parse_pair(end);
        if start.0 > end.0 || (start.0 == end.0 && start.1 > end.1) {
            Line {
                start: end,
                end: start,
            }
        } else {
            Line { start, end }
        }
    }
}

fn do_line(points: &mut HashMap<(u32, u32), u32>, line: Line) {
    let x = line.start.0;
    let y = line.start.1;
    if x == line.end.0 {
        // vertical
        for i in y..=(line.end.1) {
            *points.entry((x, i)).or_insert(0) += 1;
        }
    } else if y == line.end.1 {
        // horizontal
        for i in x..=(line.end.0) {
            *points.entry((i, y)).or_insert(0) += 1;
        }
    } else if x < line.end.0 && y < line.end.1 {
        // positive horizontal
        for (xi, yi) in (line.start.0..=line.end.0).zip(line.start.1..=line.end.1) {
            *points.entry((xi, yi)).or_insert(0) += 1;
        }
    } else {
        // negative horizontal
        let mut y = y as i64;
        for xi in line.start.0..=line.end.0 {
            *points.entry((xi, y as u32)).or_insert(0) += 1;
            y -= 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let mut points = HashMap::new();
    for line in input.lines().map(Result::unwrap).map(Line::from_str) {
        // dbg!(&line);
        do_line(&mut points, line);
    }
    let res = points
        .iter()
        .filter(|&(_, &v)| v >= 2)
        // .inspect(|x| println!("made it through filter: {:?}", x))
        .count();
    println!("Result = {}", res);
    Ok(())
}
