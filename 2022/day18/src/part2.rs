use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    fn possible_adjacents(&self) -> impl Iterator<Item = Point> + '_ {
        let deltas = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];
        deltas.into_iter().map(|(dx, dy, dz)| Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        let x = coords
            .next()
            .ok_or(ParsePointError)?
            .parse()
            .map_err(|_| ParsePointError)?;
        let y = coords
            .next()
            .ok_or(ParsePointError)?
            .parse()
            .map_err(|_| ParsePointError)?;
        let z = coords
            .next()
            .ok_or(ParsePointError)?
            .parse()
            .map_err(|_| ParsePointError)?;
        Ok(Point { x, y, z })
    }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let points: HashSet<Point> = input.lines()
        .map(Result::unwrap)
        .map(|l| l.parse())
        .map(Result::unwrap)
        .collect();

    let mut queue = vec![Point::default()];
    let range = -20..40;
    let mut seen = HashSet::new();
    let mut res = 0;

    while let Some(point) = queue.pop() {
        for neighbour in point.possible_adjacents() {
            if points.contains(&neighbour) {
                res += 1;
            } else if !seen.contains(&neighbour) && range.contains(&point.x) && range.contains(&point.y) && range.contains(&point.z) {
                queue.push(neighbour);
                seen.insert(neighbour);
            }
        }
    }

    println!("Result = {res}");
    Ok(())
}
