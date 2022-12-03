use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut res: usize = 0;

    for line in input.lines().map(Result::unwrap) {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let first_half = &bytes[..len/2];
        let second_half = &bytes[len/2..];
        for byte in first_half {
            if second_half.contains(byte) {
                res += {
                    if b'a' <= *byte &&  *byte <= b'z' {
                        usize::from(byte - b'a' + 1)
                    } else {
                        usize::from(byte - b'A' + 27)
                    }
                };
                break;
            }
        }
    }

    println!("Result = {}", res);
    Ok(())
}
