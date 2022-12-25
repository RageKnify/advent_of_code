use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_snafu<S: AsRef<str>>(snafu: S) -> isize {
    let mut res = 0;

    for c in snafu.as_ref().as_bytes().iter() {
        res *= 5;
        match c {
            b'2' => res += 2,
            b'1' => res += 1,
            b'0' => res += 0,
            b'-' => res -= 1,
            b'=' => res -= 2,
            _ => unreachable!(),
        }
    }

    res
}

fn to_snafu(mut num: isize) -> String {
    let mut buf = vec![];
    while num > 0 {
        let fit = num % 5;
        num /= 5;
        match fit {
            2 => buf.push(b'2'),
            1 => buf.push(b'1'),
            0 => buf.push(b'0'),
            3 => {
                buf.push(b'=');
                num += 1;
            }
            4 => {
                buf.push(b'-');
                num += 1;
            }
            _ => unreachable!(),
        }
    }
    buf.reverse();
    unsafe { String::from_utf8_unchecked(buf) }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let sum = input.lines().map(Result::unwrap).map(parse_snafu).sum();

    let res = to_snafu(sum);

    println!("Result = {}", res);
    Ok(())
}
