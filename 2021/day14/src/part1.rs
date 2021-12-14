use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn step(line: &Vec<u8>, conversion: &HashMap<(u8, u8), u8>) -> Vec<u8> {
    let mut res = Vec::new();
    let mut it = line.iter();
    let mut prev = *it.next().unwrap();
    res.push(prev);
    for curr in it {
        let mid = conversion[&(prev, *curr)];
        res.push(mid);
        prev = *curr;
        res.push(prev)
    }
    res
}

fn get_res(line: &Vec<u8>) -> usize {
    let mut m = HashMap::new();
    for c in line {
        *m.entry(*c).or_default() += 1usize;
    }
    let least = m
        .values()
        .reduce(|acc: &usize, item: &usize| if acc < item { acc } else { item })
        .unwrap();
    let most = m
        .values()
        .reduce(|acc: &usize, item: &usize| if acc > item { acc } else { item })
        .unwrap();
    most - least
}

fn main() -> std::io::Result<()> {
    let mut lines = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap);

    let mut line = lines.next().map(|s| s.into_bytes()).unwrap();
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

    for _ in 0..10 {
        line = step(&line, &conversion);
    }

    let res = get_res(&line);

    println!("Result = {}", res);
    Ok(())
}
