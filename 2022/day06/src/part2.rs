use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt")?);

    let mut map: HashMap<&u8, u8> = HashMap::new();
    let line = input.lines().map(Result::unwrap).next().unwrap();

    let mut add = line.as_bytes().iter().enumerate();
    let mut remove = line.as_bytes().iter();

    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;
    *map.entry(add.next().unwrap().1).or_default() += 1;

    while let Some((idx, byte)) = add.next() {
        *map.entry(byte).or_default() += 1;
        if map.len() == 14 {
            println!("{}", idx+1);
            break;
        } else {
            let key = remove.next().unwrap();
            let entry = map.entry(key).or_default();
            *entry -= 1;
            if *entry == 0 {
                map.remove(key);
            }
        }
    }

    Ok(())
}
