use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Entry {
    fn gen_translation(&self) -> HashMap<char, char> {
        // e - 4
        // b - 6
        // d - 7
        // g - 7
        // a - 8
        // c - 8
        // f - 9
        let mut res = HashMap::new();
        let mut counts: HashMap<char, u8> = HashMap::new();
        for c in self.patterns.iter().map(|s| s.chars()).flatten() {
            *counts.entry(c).or_insert(0) += 1;
        }
        // identify e, b and f
        for (&key, val) in counts.iter() {
            match val {
                4 => {
                    res.insert(key, 'e');
                }
                6 => {
                    res.insert(key, 'b');
                }
                9 => {
                    res.insert(key, 'f');
                }
                _ => (),
            }
        }
        // identify c
        {
            let pat_one = self
                .patterns
                .iter()
                .filter(|s| s.len() == 2)
                .next()
                .unwrap();
            let mut pat_one_chars = pat_one.chars();
            let p_1 = pat_one_chars.next().unwrap();
            let p_2 = pat_one_chars.next().unwrap();
            if res.get(&p_1).is_some() {
                // p_1 is f
                res.insert(p_2, 'c');
            } else {
                // p_2 is f
                res.insert(p_1, 'c');
            }
        }
        // identify a
        {
            let pat_seven = self
                .patterns
                .iter()
                .filter(|s| s.len() == 3)
                .next()
                .unwrap();
            let mut pat_seven_chars = pat_seven.chars();
            let p_1 = pat_seven_chars.next().unwrap();
            let p_2 = pat_seven_chars.next().unwrap();
            let p_3 = pat_seven_chars.next().unwrap();
            match (res.get(&p_1), res.get(&p_2)) {
                (Some('c'), Some('f')) => res.insert(p_3, 'a'),
                (Some('c'), None) => res.insert(p_2, 'a'),
                (Some('f'), Some('c')) => res.insert(p_3, 'a'),
                (Some('f'), None) => res.insert(p_2, 'a'),
                (None, _) => res.insert(p_1, 'a'),
                _ => unreachable!(),
            };
        }
        // identify d
        {
            let pat_four = self
                .patterns
                .iter()
                .filter(|s| s.len() == 4)
                .next()
                .unwrap();
            let mut pat_four_chars = pat_four.chars();
            let p_1 = pat_four_chars.next().unwrap();
            let p_2 = pat_four_chars.next().unwrap();
            let p_3 = pat_four_chars.next().unwrap();
            let p_4 = pat_four_chars.next().unwrap();
            if res.get(&p_1).is_none() {
                res.insert(p_1, 'd');
            }
            if res.get(&p_2).is_none() {
                res.insert(p_2, 'd');
            }
            if res.get(&p_3).is_none() {
                res.insert(p_3, 'd');
            }
            if res.get(&p_4).is_none() {
                res.insert(p_4, 'd');
            }
        }
        // identify g
        {
            for c in &['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
                if res.get(c).is_none() {
                    res.insert(*c, 'g');
                }
            }
        }

        res
    }

    fn decode(&self) -> usize {
        let translation = self.gen_translation();

        self.outputs
            .iter()
            .map(|o| {
                let mut v: Vec<_> = o.chars().map(|c| translation.get(&c).unwrap()).collect();
                v.sort();
                v.iter().cloned().collect()
            })
            .map(|s: String| match s.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => unreachable!(),
            })
            .reduce(|accum, item| accum * 10 + item)
            .unwrap()
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, outputs) = s.split_once(" | ").unwrap();
        let patterns = patterns.split(' ').map(String::from).collect();
        let outputs = outputs.split(' ').map(String::from).collect();
        Ok(Self { patterns, outputs })
    }
}

fn main() -> std::io::Result<()> {
    let entries: Vec<_> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|s| Entry::from_str(&s))
        .map(Result::unwrap)
        .collect();

    let res: usize = entries.iter().map(Entry::decode).sum();

    println!("Result = {}", res);
    Ok(())
}
