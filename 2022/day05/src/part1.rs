use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let file_content = read_to_string("input.txt")?;

    let (start_stacks, moves) = file_content.split_once("\n\n").unwrap();
    let (contents, numbers) = start_stacks.rsplit_once('\n').unwrap();

    let mut stacks: Vec<Vec<char>> = {
        let n_stacks = (numbers.len()+1) / 4;
        vec![vec![]; n_stacks]
    };

    for line in contents.lines() {
        for (idx, byte) in line.as_bytes().iter().skip(1).step_by(4).enumerate() {
            if *byte != b' ' {
                stacks[idx].insert(0, *byte as char);
            }
        }
    }

    for line in moves.lines() {
        for stack in &stacks {
            if let Some(letter) = stack.last() {
                print!("{letter}");
            }
        }
        println!();
        let (_, line) = line.split_once("move ").unwrap();
        let (amount, line) = line.split_once(" from ").unwrap();
        let amount: usize = amount.parse().unwrap();
        let (source, destination) = line.split_once(" to ").unwrap();
        let source: usize = source.parse::<usize>().unwrap() - 1;
        let destination: usize = destination.parse::<usize>().unwrap() - 1;

        let source_len = stacks[source].len();
        let source_slice = stacks[source][(source_len-amount)..].to_owned();
        stacks[destination].extend(source_slice.iter().rev());
        stacks[source].truncate(source_len-amount);
    }

    for stack in &stacks {
        if let Some(letter) = stack.last() {
            print!("{letter}");
        }
    }
    println!();

    Ok(())
}
