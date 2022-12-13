use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use nom::branch;
use nom::bytes::complete::tag;
use nom::combinator;
use nom::multi;
use nom::sequence;
use nom::IResult;

#[derive(PartialEq, Eq)]
enum Packet {
    Single(u64),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Single(f), Packet::Single(s)) => f.cmp(s),
            (Packet::List(lf), Packet::List(ls)) => lf.cmp(ls),
            (Packet::Single(f), Packet::List(ls)) => {
                let lf = vec![Packet::Single(*f)];
                lf.cmp(ls)
            }
            (Packet::List(lf), Packet::Single(s)) => {
                let ls = vec![Packet::Single(*s)];
                lf.cmp(&ls)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    branch::alt((
        combinator::map(nom::character::complete::u64, Packet::Single),
        combinator::map(
            sequence::delimited(
                tag("["),
                multi::separated_list0(tag(","), parse_packet),
                tag("]"),
            ),
            Packet::List,
        ),
    ))(input)
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res = 0;
    let mut idx = 1;

    let mut lines = input.lines().map(Result::unwrap);

    loop {
        let first = lines.next().unwrap();
        let (_input, first) = parse_packet(&first).unwrap();
        let second = lines.next().unwrap();
        let (_input, second) = parse_packet(&second).unwrap();

        if first < second {
            res += idx;
        }

        idx += 1;
        if lines.next().is_none() {
            break;
        }
    }

    println!("Result = {}", res);
    Ok(())
}
