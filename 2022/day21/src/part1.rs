use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum Monkey {
    Const(i64),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

impl From<&str> for Monkey {
    fn from(rest: &str) -> Self {
        use std::str::FromStr;
        match i64::from_str(rest) {
            Ok(n) => Monkey::Const(n),
            Err(_) => {
                let mut parts = rest.split(' ');
                let left = parts.next().unwrap();
                let left = u32::from_le_bytes(left.as_bytes().try_into().unwrap());
                let operator = parts.next().unwrap();
                let right = parts.next().unwrap();
                let right = u32::from_le_bytes(right.as_bytes().try_into().unwrap());
                match operator {
                    "+" => Monkey::Add(left, right),
                    "-" => Monkey::Sub(left, right),
                    "*" => Monkey::Mul(left, right),
                    "/" => Monkey::Div(left, right),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn eval(monkeys: &HashMap<u32, Monkey>) -> i64 {
    const ROOT: u32 = u32::from_le_bytes([b'r', b'o', b'o', b't']);
    let mut cache = HashMap::new();

    fn helper(monkey_id: u32, cache: &mut HashMap<u32, i64>, monkeys: &HashMap<u32, Monkey>) -> i64 {
        let entry = cache.entry(monkey_id);
        match entry {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(_) => {
                let monkey = monkeys.get(&monkey_id).unwrap();
                let value = match monkey {
                    Monkey::Const(c) => *c,
                    Monkey::Add(l, r) => {
                        let l = helper(*l, cache, monkeys);
                        let r = helper(*r, cache, monkeys);
                        l + r
                    }
                    Monkey::Sub(l, r) => {
                        let l = helper(*l, cache, monkeys);
                        let r = helper(*r, cache, monkeys);
                        l - r
                    }
                    Monkey::Mul(l, r) => {
                        let l = helper(*l, cache, monkeys);
                        let r = helper(*r, cache, monkeys);
                        l * r
                    }
                    Monkey::Div(l, r) => {
                        let l = helper(*l, cache, monkeys);
                        let r = helper(*r, cache, monkeys);
                        l / r
                    }
                };
                cache.insert(monkey_id, value);
                value
            },
        }
    }

    helper(ROOT, &mut cache, monkeys)
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut monkeys = HashMap::new();

    for line in input.lines().map(Result::unwrap) {
        let (id, rest) = line.split_once(": ").unwrap();
        let id = u32::from_le_bytes(id.as_bytes().try_into().unwrap());
        let monkey = Monkey::from(rest);
        monkeys.insert(id, monkey);
    }

    let res = eval(&monkeys);

    println!("Result = {}", res);
    Ok(())
}
