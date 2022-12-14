use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum Block {
    Rock,
    Sand,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord(usize, usize);

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

impl Coord {
    fn move_towards(&mut self, next: Coord) {
        if self.0 < next.0 {
            self.0 += 1;
        } else if self.0 > next.0 {
            self.0 -= 1;
        } else if self.1 < next.1 {
            self.1 += 1;
        } else {
            self.1 -= 1;
        }
    }

    fn below(&self) -> Self {
        let y = self.1 + 1;
        Coord(self.0, y)
    }

    fn below_left(&self) -> Self {
        let x = self.0 - 1;
        let y = self.1 + 1;
        Coord(x, y)
    }

    fn below_right(&self) -> Self {
        let x = self.0 + 1;
        let y = self.1 + 1;
        Coord(x, y)
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().map_err(|_| ())?;
        let y = y.parse().map_err(|_| ())?;
        Ok(Coord(x, y))
    }
}

fn add_line(map: &mut HashMap<Coord, Block>, line: String) {
    let mut points = line.split(" -> ").flat_map(Coord::from_str);
    let mut curr = points.next().unwrap();
    for next in points {
        while curr != next {
            map.insert(curr, Block::Rock);
            curr.move_towards(next);
        }
        map.insert(next, Block::Rock);
    }
}

fn solve(map: &mut HashMap<Coord, Block>) {
    let max_y = map.keys().map(|c| c.1).max().unwrap() + 2;
    'outer: loop {
        let mut sand_coord = Coord(500, 0);
        loop {
            if map.contains_key(&Coord(500,0)) {
                break 'outer;
            }
            if sand_coord.1 == max_y {
                map.insert(sand_coord, Block::Rock);
                break;
            }
            let below = sand_coord.below();
            if !map.contains_key(&below) {
                sand_coord = below;
            } else {
                let below_left = sand_coord.below_left();
                if !map.contains_key(&below_left) {
                    sand_coord = below_left;
                } else {
                    let below_right = sand_coord.below_right();
                    if !map.contains_key(&below_right) {
                        sand_coord = below_right;
                    } else {
                        map.insert(sand_coord, Block::Sand);
                        break;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &HashMap<Coord, Block>) {
    let min_x = map.keys().map(|c| c.0).min().unwrap();
    let max_x = map.keys().map(|c| c.0).max().unwrap();
    let max_y = map.keys().map(|c| c.1).max().unwrap();
    let mut res = String::with_capacity((max_x - min_x + 1) * max_y);
    for y in 0..=max_y {
        for x in min_x..=max_x {
            let c = match map.get(&Coord(x, y)) {
                Some(Block::Rock) => '#',
                Some(Block::Sand) => 'o',
                None => '.',
            };
            res.push(c);
        }
        res.push('\n');
    }
    println!("{res}");
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut map = HashMap::new();

    for line in input.lines().map(Result::unwrap) {
        add_line(&mut map, line);
    }

    solve(&mut map);

    let res = map.values().filter(|b| **b == Block::Sand).count();
    println!("Result = {}", res);
    Ok(())
}
