use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum MonkeyOp {
    Add(u64),
    Multiply(u64),
    Square,
}

impl MonkeyOp {
    fn handle(&self, item: u64) -> u64 {
        match self {
            MonkeyOp::Add(a) => item + a,
            MonkeyOp::Multiply(m) => item * m,
            MonkeyOp::Square => item.pow(2),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

impl FromStr for MonkeyOp {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let math = s.split_once(" = ").ok_or(ParseErr)?.1;
        let math_bytes = math.as_bytes();
        let operator = math_bytes[4];
        let second_operand = unsafe { std::str::from_utf8_unchecked(&math_bytes[6..]) };
        let second_operand = u64::from_str(second_operand);
        Ok(match (operator, second_operand) {
            (b'+', Ok(number)) => Self::Add(number),
            (b'*', Ok(number)) => Self::Multiply(number),
            (b'*', Err(_)) => Self::Square,
            _ => return Err(ParseErr),
        })
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: MonkeyOp,
    divisor: u64,
    yes: usize,
    no: usize,
    handle_count: usize,
}

impl FromStr for Monkey {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let items: Vec<u64> = lines
            .next()
            .ok_or(ParseErr)?
            .split_once(": ")
            .ok_or(ParseErr)?
            .1
            .split(", ")
            .flat_map(u64::from_str)
            .collect::<Vec<u64>>();
        let operation = lines.next().ok_or(ParseErr).and_then(MonkeyOp::from_str)?;
        let divisor: u64 = lines
            .next()
            .ok_or(ParseErr)?
            .split_once(" by ")
            .ok_or(ParseErr)?
            .1
            .parse()
            .map_err(|_| ParseErr)?;
        let yes: usize = lines
            .next()
            .ok_or(ParseErr)?
            .split_once(" monkey ")
            .ok_or(ParseErr)?
            .1
            .parse()
            .map_err(|_| ParseErr)?;
        let no: usize = lines
            .next()
            .ok_or(ParseErr)?
            .split_once(" monkey ")
            .ok_or(ParseErr)?
            .1
            .parse()
            .map_err(|_| ParseErr)?;
        Ok(Self {
            items,
            operation,
            divisor,
            yes,
            no,
            handle_count: 0,
        })
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let monkey_strs = input.split("\n\n");

    let mut monkeys: Vec<Monkey> = monkey_strs.map(|m| m.parse().unwrap()).collect();

    let magic: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = unsafe {&mut *(&mut monkeys[i] as *mut Monkey)};
            for item in monkey.items.drain(0..) {
                let post_inspection = monkey.operation.handle(item);
                let post_relief = post_inspection % magic;
                let next_monkey_idx = if post_relief % monkey.divisor == 0 {
                    monkey.yes
                } else {
                    monkey.no
                };
                let next_monkey = unsafe {&mut *(&mut monkeys[next_monkey_idx] as *mut Monkey)};
                next_monkey.items.push(post_relief);
                monkey.handle_count += 1;
            }
        }
    }

    monkeys.sort_by(|m1, m2| m1.handle_count.cmp(&m2.handle_count));
    let n_monkeys = monkeys.len();
    let res = monkeys[n_monkeys-1].handle_count * monkeys[n_monkeys-2].handle_count;

    println!("Result = {}", res);
    Ok(())
}
