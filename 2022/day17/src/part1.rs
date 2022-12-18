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
struct Rock(u32);

impl Rock {
    const fn horizontal() -> Self {
        let bottom = 0b0111_1000;
        Self(bottom)
    }
    const HORIZONTAL: Self = Self::horizontal();

    const fn cross() -> Self {
        let top = 0b0010_0000;
        let mid = 0b0111_0000;
        let bottom = 0b0010_0000;
        Self(top << 16 | mid << 8 | bottom)
    }
    const CROSS: Self = Self::cross();

    const fn corner() -> Self {
        let top = 0b0001_0000;
        let mid = 0b0001_0000;
        let bottom = 0b0111_0000;
        Self(top << 16 | mid << 8 | bottom)
    }
    const CORNER: Self = Self::corner();

    const fn vertical() -> Self {
        let a = 0b100_0000;
        let b = 0b100_0000;
        let c = 0b100_0000;
        let d = 0b100_0000;
        Self(a << 24 | b << 16 | c << 8 | d)
    }
    const VERTICAL: Self = Self::vertical();

    const fn square() -> Self {
        let top = 0b0110_0000;
        let bottom = 0b0110_0000;
        Self(top << 8 | bottom)
    }
    const SQUARE: Self = Self::square();

    fn _eprint(&self, x: usize) {
        eprintln!("<rock>");
        let line: [u8; 4] = (self.0 >> x).to_le_bytes();
        _print_map(&line);
    }
}

type Line = u8;

trait Helper {
    fn empty(&self) -> bool;
}

impl Helper for Line {
    fn empty(&self) -> bool {
        *self == 0
    }
}

fn extend_map(map: &mut Vec<Line>) -> usize {
    let y = map
        .iter()
        .enumerate()
        .rev()
        .find(|(_, l)| !l.empty())
        .map(|(idx, _)| idx + 4)
        .unwrap_or(3);
    for _ in map.len()..(y + 4) {
        map.push(Line::default());
    }
    y
}

fn field(y: usize, map: &[Line]) -> u32 {
    let bytes: [u8; 4] = <[u8; 4]>::try_from(&map[y..=y + 3]).unwrap_or([0, 0, 0, 0]);
    u32::from_le_bytes(bytes)
}

fn move_in_direction(x: &mut usize, y: usize, map: &[Line], direction: JetDirection, rock: Rock) {
    let field = field(y, map);
    if direction == JetDirection::Left {
        // left
        if *x == 0 {
            return;
        }
        if rock.0 >> (*x - 1) & field == 0 {
            *x -= 1;
        }
    } else {
        // right
        if rock.0 >> (*x) & 0x01010101 != 0 {
            // if any touch the right wall can't go right
            return;
        }
        if rock.0 >> (*x + 1) & field == 0 {
            *x += 1;
        }
    }
}

fn try_down(y: &mut usize, x: usize, map: &mut [Line], rock: Rock) -> bool {
    if *y == 0 {
        return false;
    }
    let field = field(*y - 1, map);
    if (rock.0 >> x & field) == 0 {
        *y -= 1;
        true
    } else {
        false
    }
}

fn place_piece(x: usize, y: usize, map: &mut [Line], rock: Rock) {
    let rock = rock.0 >> x;
    map[y + 3] |= (rock >> 24) as u8;
    map[y + 2] |= (rock >> 16) as u8;
    map[y + 1] |= (rock >> 8) as u8;
    map[y + 0] |= rock as u8;
}

fn map_height(map: &[Line]) -> usize {
    map.iter()
        .enumerate()
        .rev()
        .find(|(_, l)| !l.empty())
        .map(|(idx, _)| idx + 1)
        .unwrap()
}

fn _eprint_x(x: usize) {
    for _ in 0..x {
        eprint!(".");
    }
    eprintln!("");
}

fn simulate_rock(map: &mut Vec<Line>, rock: Rock, jet_movement: &mut JetMovement) {
    let mut y = extend_map(map);
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

fn simulate(
    map: &mut Vec<Line>,
    mut rocks: impl Iterator<Item = Rock>,
    jet_movement: &mut JetMovement,
) -> usize {
    for _round in 0..2022 {
        // start rock drop
        let rock = rocks.next().unwrap();
        simulate_rock(map, rock, jet_movement);

        // dbg!(_round);
        // eprintln!("<map>");
        // _print_map(&map);
    }

    map_height(&map)
}

fn _print_map(map: &[Line]) {
    let mut res = String::with_capacity(8 * map.len());
    for line in map.iter().rev().filter(|l| **l != 0 || map.len() < 5) {
        for p in (0..7).rev() {
            let mask = 2u8.pow(p);
            if line & mask == mask {
                res.push('#');
            } else {
                res.push('.')
            }
        }
        res.push('\n');
    }
    eprint!("{res}");
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
    let rocks = vec![
        Rock::HORIZONTAL,
        Rock::CROSS,
        Rock::CORNER,
        Rock::VERTICAL,
        Rock::SQUARE,
    ]
    .into_iter()
    .cycle();
    let mut map = vec![];

    let res = simulate(&mut map, rocks, &mut jet_movement);

    println!("Result = {}", res);
    Ok(())
}
