use std::{collections::HashMap, fs::read_to_string};

#[derive(Default, Debug)]
struct Dir {
    files: Vec<u64>,
    dirs: HashMap<String, Dir>,
    size: u64,
}

impl Dir {
    fn get(&mut self, path: &str) -> &mut Self {
        if path == "/" || path == "" {
            return self;
        } else {
            if let Some((top, rest)) = path.split_once("/") {
                if top == "" {
                    return self.get(rest);
                }
                return self.dirs.get_mut(top).unwrap().get(rest);
            } else {
                return self.dirs.get_mut(path).unwrap();
            }
        }
    }

    fn calculate_sizes(&mut self) {
        for child in self.dirs.values_mut() {
            child.calculate_sizes();
        }
        self.size = self
            .dirs
            .values()
            .map(|d| d.size)
            .chain(self.files.iter().cloned())
            .sum();
    }

    fn calculate_result(&self) -> u64 {
        self.dirs.values().map(Self::calculate_result).sum::<u64>()
            + if self.size <= 100_000 { self.size } else { 0 }
    }

    fn result(&mut self) -> u64 {
        self.calculate_sizes();
        self.calculate_result()
    }
}

fn main() -> std::io::Result<()> {
    let file_content = read_to_string("input.txt")?;
    let mut commands = file_content.split("$ ");
    commands.next();

    let mut root = Dir::default();
    let mut curr_path = String::from("/");

    for full_command in commands {
        let curr = root.get(&curr_path);
        let mut words = full_command.split_whitespace();
        let command = words.next().unwrap();
        if command == "cd" {
            let path = words.next().unwrap();
            if path == "/" {
                curr_path.clear();
                curr_path.push('/');
            } else if path == ".." {
                let idx = curr_path.rfind('/').unwrap();
                curr_path.truncate(idx);
            } else {
                curr_path.push('/');
                curr_path.push_str(path);
            }
        } else {
            while let Some(size_or_dir) = words.next() {
                let name = words.next().unwrap();
                if let Ok(size) = size_or_dir.parse::<u64>() {
                    curr.files.push(size);
                } else {
                    curr.dirs.insert(name.to_owned(), Dir::default());
                }
            }
        }
    }

    println!("Result = {}", root.result());
    Ok(())
}
