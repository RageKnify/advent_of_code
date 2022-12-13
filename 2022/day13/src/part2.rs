use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use nom::branch;
use nom::bytes::complete::tag;
use nom::combinator;
use nom::multi;
use nom::sequence;
use nom::IResult;

#[derive(PartialEq, Eq, Clone)]
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

    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Single(2)])]);
    let divider6 = Packet::List(vec![Packet::List(vec![Packet::Single(6)])]);

    let mut packets = vec![divider2.clone(), divider6.clone()];

    input.lines().map(Result::unwrap).for_each(|line| {
        if let Ok((_inp, packet)) = parse_packet(&line) {
            packets.push(packet);
        }
    });

    packets.sort();

    let res = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet == divider2 || **packet == divider6)
        .map(|(idx, _)| idx + 1)
        .product::<usize>();

    println!("Result = {}", res);
    Ok(())
}
