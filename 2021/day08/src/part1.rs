use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

struct Entry {
    outputs: Vec<String>,
}

impl Entry {
    fn count_easy(&self) -> usize {
        fn is_easy(pattern: &str) -> bool {
            match pattern.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            }
        }
        self.outputs.iter().filter(|s| is_easy(s)).count()
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, outputs) = s.split_once(" | ").unwrap();
        let outputs = outputs.split(' ').map(String::from).collect();
        Ok(Self { outputs })
    }
}

fn main() -> std::io::Result<()> {
    let entries: Vec<_> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .map(|s| Entry::from_str(&s))
        .map(Result::unwrap)
        .collect();

    let res: usize = entries.iter().map(Entry::count_easy).sum();

    println!("Result = {}", res);
    Ok(())
}
