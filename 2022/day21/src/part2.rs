use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

const ROOT: u32 = u32::from_le_bytes([b'r', b'o', b'o', b't']);
const HUMN: u32 = u32::from_le_bytes([b'h', b'u', b'm', b'n']);

#[derive(Clone, Copy)]
enum Monkey {
    Human,
    Root(u32, u32),
    Const(i64),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

impl Monkey {
    fn root_from_rest(rest: &str) -> Self {
        let mut parts = rest.split(' ');
        let left = parts.next().unwrap();
        let left = u32::from_le_bytes(left.as_bytes().try_into().unwrap());
        let _ = parts.next().unwrap();
        let right = parts.next().unwrap();
        let right = u32::from_le_bytes(right.as_bytes().try_into().unwrap());
        Monkey::Root(left, right)
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum AST {
    Human,
    Root(Box<AST>, Box<AST>),
    Const(i64),
    Add(Rc<AST>, Rc<AST>),
    Sub(Rc<AST>, Rc<AST>),
    Mul(Rc<AST>, Rc<AST>),
    Div(Rc<AST>, Rc<AST>),
}

impl AST {
    fn order_root(self) -> (AST, AST) {
        match self {
            AST::Root(l, r) => {
                let l = *l;
                let r = *r;
                match &l {
                    AST::Const(_) => (l, r),
                    _ => (r, l),
                }
            }
            _ => unreachable!(),
        }
    }

    fn unwrap_constant(&self) -> i64 {
        if let AST::Const(constant) = *self {
            constant
        } else {
            panic!("Tried to get const from non const AST")
        }
    }
}

impl Add<AST> for AST {
    type Output = AST;

    fn add(self, rhs: AST) -> Self::Output {
        match (&self, &rhs) {
            (AST::Const(l), AST::Const(r)) => AST::Const(l + r),
            _ => AST::Add(Rc::new(self), Rc::new(rhs)),
        }
    }
}

impl Sub<AST> for AST {
    type Output = AST;

    fn sub(self, rhs: AST) -> Self::Output {
        match (&self, &rhs) {
            (AST::Const(l), AST::Const(r)) => AST::Const(l - r),
            _ => AST::Sub(Rc::new(self), Rc::new(rhs)),
        }
    }
}

impl Mul<AST> for AST {
    type Output = AST;

    fn mul(self, rhs: AST) -> Self::Output {
        match (&self, &rhs) {
            (AST::Const(l), AST::Const(r)) => AST::Const(l * r),
            _ => AST::Mul(Rc::new(self), Rc::new(rhs)),
        }
    }
}

impl Div<AST> for AST {
    type Output = AST;

    fn div(self, rhs: AST) -> Self::Output {
        match (&self, &rhs) {
            (AST::Const(l), AST::Const(r)) => AST::Const(l / r),
            _ => AST::Div(Rc::new(self), Rc::new(rhs)),
        }
    }
}

fn advance(constant: i64, ast: AST) -> (i64, AST) {
    fn advance_add(constant: i64, left: Rc<AST>, right: Rc<AST>) -> (i64, AST) {
        match *left {
            AST::Const(left_constant) => {
                // lc + r = c
                // r = c - lc
                (constant - left_constant, (*right).clone())
            }
            _ => {
                // l + rc = c
                // l = c - rc
                let right_constant = right.unwrap_constant();
                (constant - right_constant, (*left).clone())
            }
        }
    }
    fn advance_sub(constant: i64, left: Rc<AST>, right: Rc<AST>) -> (i64, AST) {
        match *left {
            AST::Const(left_constant) => {
                // lc - r  = c
                // lc - c = r
                (left_constant - constant, (*right).clone())
            }
            _ => {
                // l - rc = c
                // l = c + rc
                let right_constant = right.unwrap_constant();
                (constant + right_constant, (*left).clone())
            }
        }
    }
    fn advance_mul(constant: i64, left: Rc<AST>, right: Rc<AST>) -> (i64, AST) {
        match *left {
            AST::Const(left_constant) => {
                // c = lc * r
                // r = c / lc
                (constant / left_constant, (*right).clone())
            }
            _ => {
                // c = l * rc
                // l = c / rc
                let right_constant = right.unwrap_constant();
                (constant / right_constant, (*left).clone())
            }
        }
    }
    fn advance_div(constant: i64, left: Rc<AST>, right: Rc<AST>) -> (i64, AST) {
        match *left {
            AST::Const(left_constant) => {
                // c = lc / r
                // r = lc / c
                (left_constant / constant, (*right).clone())
            }
            _ => {
                // c = l / rc
                // l = c * rc
                let right_constant = right.unwrap_constant();
                (constant * right_constant, (*left).clone())
            }
        }
    }
    match ast {
        AST::Add(l, r) => advance_add(constant, l, r),
        AST::Sub(l, r) => advance_sub(constant, l, r),
        AST::Mul(l, r) => advance_mul(constant, l, r),
        AST::Div(l, r) => advance_div(constant, l, r),
        AST::Const(_) | AST::Human | AST::Root(_, _) => unreachable!(),
    }
}

fn eval(monkeys: &HashMap<u32, Monkey>) -> i64 {
    let mut cache = HashMap::new();

    fn helper(
        monkey_id: u32,
        cache: &mut HashMap<u32, AST>,
        monkeys: &HashMap<u32, Monkey>,
    ) -> AST {
        let entry = cache.entry(monkey_id);
        match entry {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(_) => {
                let monkey = monkeys.get(&monkey_id).unwrap();
                let value = match monkey {
                    Monkey::Const(c) => AST::Const(*c),
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
                    Monkey::Human => AST::Human,
                    Monkey::Root(l, r) => {
                        let l = helper(*l, cache, monkeys);
                        let r = helper(*r, cache, monkeys);
                        AST::Root(Box::new(l), Box::new(r))
                    }
                };
                cache.insert(monkey_id, value.clone());
                value
            }
        }
    }

    let root = helper(ROOT, &mut cache, monkeys);

    fn make_equal(root: AST) -> i64 {
        let (constant, mut symbolic) = root.order_root();
        let mut constant = constant.unwrap_constant();

        while symbolic != AST::Human {
            let pair = advance(constant, symbolic);
            constant = pair.0;
            symbolic = pair.1;
        }

        constant
    }

    make_equal(root)
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut monkeys = HashMap::new();

    for line in input.lines().map(Result::unwrap) {
        let (id, rest) = line.split_once(": ").unwrap();
        let id = u32::from_le_bytes(id.as_bytes().try_into().unwrap());
        let monkey = if id == HUMN {
            Monkey::Human
        } else if id == ROOT {
            Monkey::root_from_rest(rest)
        } else {
            Monkey::from(rest)
        };
        monkeys.insert(id, monkey);
    }

    let res = eval(&monkeys);

    println!("Result = {}", res);
    Ok(())
}
