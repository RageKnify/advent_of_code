use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type LineMap = HashMap<(u8, u8), usize>;

fn line_to_map(line: &Vec<u8>) -> LineMap {
    let mut it = line.iter();
    let mut prev = it.next().unwrap();
    let mut res = LineMap::new();
    for curr in it {
        *res.entry((*prev, *curr)).or_default() += 1;
        prev = curr;
    }
    res
}

fn step(line_map: &LineMap, conversion: &HashMap<(u8, u8), u8>) -> LineMap {
    let mut res = LineMap::new();
    for (pair, count) in line_map.iter() {
        let (prev, next) = pair;
        let mid = conversion[pair];
        *res.entry((*prev, mid)).or_default() += count;
        *res.entry((mid, *next)).or_default() += count;
    }
    res
}

fn get_res(line_map: &LineMap, last: u8) -> usize {
    let mut counts = HashMap::new();
    counts.insert(last, 1);
    for (pair, count) in line_map.iter() {
        let (prev, _next) = pair;
        *counts.entry(*prev).or_default() += count;
        // *counts.entry(_next).or_default() += count;
    }
    let least = counts
        .values()
        .reduce(|acc: &usize, item: &usize| if acc < item { acc } else { item })
        .unwrap();
    let most = counts
        .values()
        .reduce(|acc: &usize, item: &usize| if acc > item { acc } else { item })
        .unwrap();
    dbg!(least, most);
    most - least
}

fn main() -> std::io::Result<()> {
    let mut lines = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap);

    let line = lines.next().map(|s| s.into_bytes()).unwrap();
    let mut line_map = line_to_map(&line);
    lines.next();

    let mut conversion = HashMap::new();

    for l in lines {
        let (seq, ins) = l.split_once(" -> ").unwrap();
        let seq = {
            let seq_bytes = seq.as_bytes();
            (seq_bytes[0], seq_bytes[1])
        };
        let ins = ins.as_bytes()[0];
        conversion.insert(seq, ins);
    }

    for _ in 0..40 {
        line_map = step(&line_map, &conversion);
    }

    let res = get_res(&line_map, *line.last().unwrap());

    println!("Result = {}", res);
    Ok(())
}
