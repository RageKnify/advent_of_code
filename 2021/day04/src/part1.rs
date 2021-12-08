use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

struct Board {
    inner: [[u8; 5]; 5],
}

impl FromStr for Board {
    type Err = ();

    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner: {
                let mut it = lines.split('\n').map(|line| {
                    let mut it = line
                        .split_whitespace()
                        .map(u8::from_str)
                        .map(Result::unwrap);
                    [
                        it.next().unwrap(),
                        it.next().unwrap(),
                        it.next().unwrap(),
                        it.next().unwrap(),
                        it.next().unwrap(),
                    ]
                });
                [
                    it.next().unwrap(),
                    it.next().unwrap(),
                    it.next().unwrap(),
                    it.next().unwrap(),
                    it.next().unwrap(),
                ]
            },
        })
    }
}

impl Board {
    fn place(&mut self, number: u8) -> bool {
        let mut flag = false;
        for l in self.inner.iter_mut() {
            for n in l.iter_mut() {
                if *n == number {
                    *n = 255;
                    if !flag {
                        flag = true;
                    }
                }
            }
        }
        if flag {
            if self.inner.iter().any(|l| l.iter().all(|&x| x == 255)) {
                // line
                return true;
            }
            if (0..5).any(|idx| self.inner.iter().map(|l| l[idx]).all(|x| x == 255)) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        self.inner
            .iter()
            .map(|l| {
                l.iter()
                    .filter(|&&x| x != 255)
                    .cloned()
                    .map(usize::from)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

fn get_res(boards: &mut Vec<Board>, numbers: &[u8]) -> usize {
    for n in numbers {
        for b in boards.iter_mut() {
            if b.place(*n) {
                return b.score() * *n as usize;
            }
        }
    }
    unreachable!()
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let mut lines = input.lines().map(Result::unwrap);

    let numbers: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(u8::from_str)
        .map(Result::unwrap)
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut five_lines = lines.next().unwrap();
        for _ in 0..4 {
            five_lines.push('\n');
            five_lines.push_str(&lines.next().unwrap());
        }
        let board = Board::from_str(&five_lines).unwrap();
        boards.push(board);
    }

    let res = get_res(&mut boards, &numbers);

    println!("Result = {}", res);
    Ok(())
}
