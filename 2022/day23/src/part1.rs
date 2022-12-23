use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&mut self) {
        use Direction::*;
        *self = match self {
            North => South,
            South => West,
            West => East,
            East => North,
        };
    }

    fn three_from_elf(&self, elf: &Coord) -> [Coord; 3] {
        let deltas = match self {
            Direction::North => [(-1, -1), (0, -1), (1, -1)],
            Direction::South => [(-1, 1), (0, 1), (1, 1)],
            Direction::West => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::East => [(1, -1), (1, 0), (1, 1)],
        };
        deltas.map(|(dx, dy)| (elf.0 + dx, elf.1 + dy))
    }

    fn advance(&self, elf: &Coord) -> Coord {
        let (mut x, mut y) = elf;
        match self {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
            Direction::East => x += 1,
        }
        (x, y)
    }
}

type Coord = (i32, i32);

fn calculate_result(coords: &HashSet<Coord>) -> i32 {
    let min_x = coords.iter().map(|(x, _)| x).min().cloned().unwrap();
    let max_x = coords.iter().map(|(x, _)| x).max().cloned().unwrap();
    let min_y = coords.iter().map(|(_, y)| y).min().cloned().unwrap();
    let max_y = coords.iter().map(|(_, y)| y).max().cloned().unwrap();
    let mut res = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !coords.contains(&(x, y)) {
                res += 1;
            }
        }
    }
    res
}

fn eight_adjacent(elf: Coord) -> [Coord; 8] {
    const DELTAS: [Coord; 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    DELTAS.map(|(dx, dy)| (elf.0 + dx, elf.1 + dy))
}

fn advance_round(coords: &mut HashSet<Coord>, direction: Direction) {
    let mut proposals = HashMap::new();
    let mut n_proposers: HashMap<Coord, usize> = HashMap::new();
    // first half
    for elf in coords.iter() {
        // if no adjacent elves continue
        if eight_adjacent(*elf).iter().all(|pos| !coords.contains(pos)) {
            continue;
        }

        let mut elfs_direction = direction;
        for _ in 0..4 {
            let three_pos = elfs_direction.three_from_elf(elf);
            if three_pos.iter().all(|pos| !coords.contains(pos)) {
                // can take this direction
                let pos = elfs_direction.advance(elf);
                proposals.insert(*elf, pos);
                *n_proposers.entry(pos).or_default() += 1;
                break;
            }
            elfs_direction.next();
        }
    }
    // second half
    for (original, destination) in proposals.iter() {
        if *n_proposers.entry(*destination).or_default() == 1 {
            coords.remove(original);
            coords.insert(*destination);
        }
    }
}

fn _print_map(coords: &HashSet<(i32, i32)>) {
    let min_y = -2;
    let max_y = 9;
    let dist_y = max_y - min_y + 1;

    let min_x = -3;
    let max_x = 10;
    let dist_x = max_x - min_x + 1;

    let mut bools = vec![vec![false; dist_x as usize]; dist_y as usize];
    for (x, y) in coords {
        let idx_y = (y - min_y) as usize;
        let idx_x = (x - min_x) as usize;
        bools[idx_y][idx_x] = true;
    }

    let mut buf = String::with_capacity(dist_y as usize * (dist_x + 1) as usize);
    for line in bools {
        for b in line {
            if b {
                buf.push('#')
            } else {
                buf.push('.')
            }
        }
        buf.push('\n')
    }
    eprintln!("{buf}");
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut coords: HashSet<Coord> = HashSet::new();

    for (y, line) in input.lines().map(Result::unwrap).enumerate() {
        let bytes = line.as_bytes();
        coords.extend(
            bytes
                .iter()
                .enumerate()
                .map(|(x, byte)| (x, y, byte))
                .filter_map(|(x, y, byte)| {
                    if *byte == b'#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                }),
        )
    }


    const ROUNDS: usize = 10;
    let mut direction = Direction::North;
    for _round in 0..ROUNDS {
        advance_round(&mut coords, direction);
        direction.next();
    }

    let res = calculate_result(&coords);
    println!("Result = {}", res);
    Ok(())
}
