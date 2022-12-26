use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i8, i8);

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.0).field(&self.1).finish()
    }
}

impl From<(i8, i8)> for Coord {
    fn from(c: (i8, i8)) -> Self {
        Coord(c.0, c.1)
    }
}

impl From<Coord> for (i8, i8) {
    fn from(other: Coord) -> Self {
        (other.0, other.1)
    }
}

impl Coord {
    fn advance(&self, direction: Direction, len_x: i8, len_y: i8) -> Coord {
        let Coord(mut x, mut y) = *self;
        match direction {
            Direction::North => {
                y -= 1;
            }
            Direction::South => {
                y += 1;
            }
            Direction::East => {
                x += 1;
            }
            Direction::West => {
                x -= 1;
            }
        }
        if x == 0 {
            x = len_x - 2;
        } else if x == len_x - 1 {
            x = 1;
        }
        if y == 0 {
            y = len_y - 2;
        } else if y == len_y - 1 {
            if x != len_x - 1 {
                y = 1;
            }
        } else if y == len_y {
            y = 1;
        }
        (x, y).into()
    }

    fn is_wall(&self, len_x: i8, len_y: i8) -> bool {
        if self.0 <= 0 {
            return true;
        }
        if self.0 == len_x - 1 {
            return true;
        }
        if self.1 == 0 && self.0 == 1 {
            return false;
        }
        if self.1 <= 0 {
            return true;
        }
        if self.1 == len_y - 1 && self.0 == len_x - 2 {
            return false;
        }
        self.1 >= len_y - 1
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<Direction> for u8 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => b'^',
            Direction::South => b'v',
            Direction::East => b'>',
            Direction::West => b'<',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    g: u32,
    blizzards: Vec<(Coord, Direction)>,
    my_position: Coord,
    len_x: i8,
    len_y: i8,
    end: Coord,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct StateHashable {
    my_position: Coord,
    g: u32,
}

impl From<&State> for StateHashable {
    fn from(state: &State) -> Self {
        StateHashable { my_position: state.my_position, g: state.g }
    }
}

impl State {
    fn top_left(&self) -> Coord {
        (1, 0).into()
    }

    fn bottom_right(&self) -> Coord {
        (self.len_x - 2, self.len_y - 1).into()
    }

    fn set_end(&mut self, new_end:Coord) {
        self.end = new_end;
    }

    fn end(&self) -> Coord {
        self.end
    }

    fn manhattan_distance(&self) -> i16 {
        let end = self.end();
        let dist_x = end.0 - self.my_position.0;
        let dist_y = end.1 - self.my_position.1;
        dist_x.abs() as i16 + dist_y.abs() as i16
    }

    fn f(&self) -> i16 {
        self.g as i16 + self.manhattan_distance()
    }

    fn expand(&self) -> Vec<State> {
        const DELTAS: [(i8, i8); 5] = [(-1, 0), (1, 0), (0, 0), (0, -1), (0, 1)];
        let pos: (i8, i8) = self.my_position.into();
        let g = self.g + 1;
        let new_blizzards: Vec<_> = self
            .blizzards
            .iter()
            .map(|(c, d)| (c.advance(*d, self.len_x, self.len_y), *d))
            .collect();
        DELTAS
            .map(|d| (pos.0 + d.0, pos.1 + d.1))
            .iter()
            .cloned()
            .map(Coord::from)
            .filter(|new_pos| !new_pos.is_wall(self.len_x, self.len_y))
            .filter(|new_pos| new_blizzards.iter().all(|b| b.0 != *new_pos))
            .map(|new_pos| State {
                blizzards: new_blizzards.clone(),
                my_position: new_pos,
                g,
                ..*self
            })
            .collect()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f().cmp(&other.f()).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_map<S: AsRef<str>>(string: S) -> State {
    fn get_x(string: &str) -> i8 {
        string.lines().next().unwrap().len() as i8
    }
    let len_x = get_x(string.as_ref());
    let bytes = string.as_ref().as_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut blizzards = vec![];
    for b in bytes {
        match b {
            b'\n' => {
                y += 1;
                x = 0;
                continue;
            }
            b'^' => blizzards.push(((x, y).into(), Direction::North)),
            b'v' => blizzards.push(((x, y).into(), Direction::South)),
            b'>' => blizzards.push(((x, y).into(), Direction::East)),
            b'<' => blizzards.push(((x, y).into(), Direction::West)),
            _ => (),
        }
        x += 1;
    }
    let len_y = y;
    let my_position = (1, 0).into();
    let g = 0;
    let end = (len_x - 2, len_y - 1).into();
    State {
        blizzards,
        my_position,
        len_x,
        len_y,
        g,
        end,
    }
}

fn solve(start: State) -> State {
    let mut queue = BinaryHeap::new();
    let mut found: HashSet<StateHashable> = HashSet::new();
    found.insert((&start).into());
    queue.push(start);

    loop {
        let state = {
            let opt_state = queue.pop();
            match opt_state {
                Some(val) => val,
                None => {
                    panic!();
                }
            }
        };
        if state.manhattan_distance() == 0 {
            return state;
        }
        for new_state in state.expand() {
            let new_state_hashed: StateHashable = (&new_state).into();
            if !found.contains(&new_state_hashed) {
                found.insert(new_state_hashed);
                queue.push(new_state)
            }
        }
    };
}

fn main() -> std::io::Result<()> {
    let file_content = read_to_string("input.txt")?;

    let start = parse_map(file_content);

    let mut end = solve(start);

    end.set_end(end.top_left());

    let mut back_to_start = solve(end);

    back_to_start.set_end(back_to_start.bottom_right());

    let back_to_end = solve(back_to_start);

    println!("Result: {}", back_to_end.g);
    Ok(())
}
