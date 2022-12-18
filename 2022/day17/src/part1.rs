use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JetDirection {
    Left,
    Right,
}

impl From<u8> for JetDirection {
    fn from(src: u8) -> Self {
        match src {
            b'<' => Self::Left,
            b'>' => Self::Right,
            _ => panic!("unexpected direction"),
        }
    }
}

struct JetMovement {
    directions: Box<[JetDirection]>,
    idx: usize,
}

impl Iterator for JetMovement {
    type Item = JetDirection;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.idx < self.directions.len() {
            self.directions.get(self.idx).cloned()
        } else {
            self.idx = 0;
            self.directions.get(self.idx).cloned()
        };
        self.idx += 1;
        return res;
    }
}

impl From<&str> for JetMovement {
    fn from(src: &str) -> Self {
        let directions = src
            .as_bytes()
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<JetDirection>>()
            .into();
        let idx = 0;
        Self { directions, idx }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RockGenerator {
    last: Rock,
}

impl Default for RockGenerator {
    fn default() -> Self {
        RockGenerator { last: Rock::Square }
    }
}

impl Iterator for RockGenerator {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = match self.last {
            Rock::Horizontal => Rock::Cross,
            Rock::Cross => Rock::Corner,
            Rock::Corner => Rock::Vertical,
            Rock::Vertical => Rock::Square,
            Rock::Square => Rock::Horizontal,
        };
        return Some(self.last);
    }
}

type Line = [bool; 7];

trait Helper {
    fn empty(&self) -> bool;
}

impl Helper for Line {
    fn empty(&self) -> bool {
        self[0] == false
            && self[1] == false
            && self[2] == false
            && self[3] == false
            && self[4] == false
            && self[5] == false
            && self[6] == false
    }
}

fn extend_map(map: &mut Vec<Line>, rock: Rock) -> usize {
    let highest = map
        .iter()
        .enumerate()
        .rev()
        .find(|(_, l)| !l.empty())
        .map(|(idx, _)| idx);
    let to_add = match highest {
        Some(idx) => (idx + 3).checked_sub(map.len()).unwrap_or(0),
        None => 3,
    } + match rock {
        Rock::Horizontal => 1,
        Rock::Cross => 3,
        Rock::Corner => 3,
        Rock::Vertical => 4,
        Rock::Square => 2,
    };
    for _ in 0..to_add {
        map.push(Line::default());
    }
    highest.map(|h| h + 4).unwrap_or(3)
}

fn move_in_direction(x: &mut usize, y: usize, map: &[Line], direction: JetDirection, rock: Rock) {
    if direction == JetDirection::Left {
        // left
        if *x == 0 {
            return;
        }
        match rock {
            Rock::Horizontal => {
                if !map[y][*x - 1] {
                    *x -= 1;
                }
            }
            Rock::Cross => {
                if !(map[y][*x] || map[y + 1][*x - 1] || map[y + 2][*x]) {
                    *x -= 1;
                }
            }
            Rock::Corner => {
                if !(map[y][*x - 1] || map[y + 1][*x + 1] || map[y + 2][*x + 1]) {
                    *x -= 1;
                }
            }
            Rock::Vertical => {
                if !(map[y][*x - 1]
                    || map[y + 1][*x - 1]
                    || map[y + 2][*x - 1]
                    || map[y + 3][*x - 1])
                {
                    *x -= 1;
                }
            }
            Rock::Square => {
                if !(map[y][*x - 1] || map[y + 1][*x - 1]) {
                    *x -= 1;
                }
            }
        }
    } else {
        // right
        if *x == 6 {
            return;
        }
        match rock {
            Rock::Horizontal => {
                if *x <= 2 && !map[y][*x + 4] {
                    *x += 1;
                }
            }
            Rock::Cross => {
                if *x <= 3 && !(map[y][*x + 2] || map[y + 1][*x + 3] || map[y + 2][*x + 2]) {
                    *x += 1;
                }
            }
            Rock::Corner => {
                if *x <= 3 && !(map[y][*x + 3] || map[y + 1][*x + 3] || map[y + 2][*x + 3]) {
                    *x += 1;
                }
            }
            Rock::Vertical => {
                if *x <= 5
                    && !(map[y][*x + 1]
                        || map[y + 1][*x + 1]
                        || map[y + 2][*x + 1]
                        || map[y + 3][*x + 1])
                {
                    *x += 1;
                }
            }
            Rock::Square => {
                if *x <= 4 && !(map[y][*x + 2] || map[y + 1][*x + 2]) {
                    *x += 1;
                }
            }
        }
    }
}

fn try_down(y: &mut usize, x: usize, map: &mut [Line], rock: Rock) -> bool {
    if *y == 0 {
        return false;
    }
    match rock {
        Rock::Horizontal => {
            if !(map[*y - 1][x] || map[*y - 1][x + 1] || map[*y - 1][x + 2] || map[*y - 1][x + 3]) {
                *y -= 1;
                true
            } else {
                false
            }
        }
        Rock::Cross => {
            if !(map[*y - 1][x + 1] || map[*y][x] || map[*y][x + 2]) {
                *y -= 1;
                true
            } else {
                false
            }
        }
        Rock::Corner => {
            if !(map[*y - 1][x] || map[*y - 1][x + 1] || map[*y - 1][x + 2]) {
                *y -= 1;
                true
            } else {
                false
            }
        }
        Rock::Vertical => {
            if !(map[*y - 1][x]) {
                *y -= 1;
                true
            } else {
                false
            }
        }
        Rock::Square => {
            if !(map[*y - 1][x] || map[*y - 1][x + 1]) {
                *y -= 1;
                true
            } else {
                false
            }
        }
    }
}

fn place_piece(x: usize, y: usize, map: &mut [Line], rock: Rock) {
    match rock {
        Rock::Horizontal => {
            map[y][x] = true;
            map[y][x + 1] = true;
            map[y][x + 2] = true;
            map[y][x + 3] = true;
        }
        Rock::Cross => {
            map[y + 1][x] = true;
            map[y + 0][x + 1] = true;
            map[y + 1][x + 1] = true;
            map[y + 2][x + 1] = true;
            map[y + 1][x + 2] = true;
        }
        Rock::Corner => {
            map[y][x] = true;
            map[y][x + 1] = true;
            map[y][x + 2] = true;
            map[y + 1][x + 2] = true;
            map[y + 2][x + 2] = true;
        }
        Rock::Vertical => {
            map[y][x] = true;
            map[y + 1][x] = true;
            map[y + 2][x] = true;
            map[y + 3][x] = true;
        }
        Rock::Square => {
            map[y][x] = true;
            map[y][x + 1] = true;
            map[y + 1][x] = true;
            map[y + 1][x + 1] = true;
        }
    }
}

fn map_height(map: &[Line]) -> usize {
    map.iter()
        .enumerate()
        .rev()
        .find(|(_, l)| !l.empty())
        .map(|(idx, _)| idx + 1)
        .unwrap()
}

fn simulate(
    map: &mut Vec<Line>,
    mut rocks: RockGenerator,
    jet_movement: &mut JetMovement,
) -> usize {
    for _round in 0..2022 {
        // start rock drop
        let rock = rocks.next().unwrap();
        let mut y = extend_map(map, rock);
        let mut x = 2usize;
        loop {
            let direction = jet_movement.next().unwrap();
            move_in_direction(&mut x, y, &map, direction, rock);
            let went_down = try_down(&mut y, x, map, rock);
            if !went_down {
                place_piece(x, y, map, rock);
                break;
            }
        }
    }

    map_height(&map)
}

fn _print_map(map: &[[bool; 7]]) {
    let mut res = String::with_capacity(7 * map.len());
    for line in map.iter().rev().filter(|l| !l.empty()) {
        for b in line {
            if *b {
                res.push('#');
            } else {
                res.push('.')
            }
        }
        res.push('\n');
    }
    print!("{res}");
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);
    let mut jet_movement: JetMovement = input
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .as_str()
        .into();
    let rocks = RockGenerator::default();
    let mut map = vec![];

    let res = simulate(&mut map, rocks, &mut jet_movement);

    println!("Result = {}", res);
    Ok(())
}
