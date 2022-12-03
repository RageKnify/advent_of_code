use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res: usize = 0;
    let mut set: HashSet<u8> = HashSet::new();
    let mut i = 0;

    for line in input.lines().map(Result::unwrap) {
        let bytes = line.as_bytes();

        if i%3 == 0 {
            for byte in &set {
                res += {
                    if b'a' <= *byte &&  *byte <= b'z' {
                        usize::from(byte - b'a' + 1)
                    } else {
                        usize::from(byte - b'A' + 27)
                    }
                };
            }
            set.clear();
            set.extend(bytes);
            i = 1;
        } else {
            set.retain(|b| bytes.contains(b));
            i += 1;
        }
    }
    for byte in &set {
        res += {
            if b'a' <= *byte &&  *byte <= b'z' {
                usize::from(byte - b'a' + 1)
            } else {
                usize::from(byte - b'A' + 27)
            }
        };
    }

    println!("Result = {}", res);
    Ok(())
}
