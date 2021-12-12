use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Graph(HashMap<String, Vec<String>>);

fn is_lower(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn get_res(graph: &Graph) -> usize {
    fn recursive<'graph, 's: 'graph>(
        curr: &'s str,
        graph: &'graph Graph,
        visited: &mut HashSet<&'graph str>,
    ) -> usize {
        let mut res = 0;
        let next_ones = &graph.0[curr];
        if is_lower(curr) {
            visited.insert(curr);
        }
        for n in next_ones {
            if n == "end" {
                res += 1;
            } else if !visited.contains(n.as_str()) {
                res += recursive(n, graph, &mut visited.clone());
            }
        }
        res
    }

    let mut res = 0;
    let starts = &graph.0["start"];
    let mut visited = HashSet::new();
    visited.insert("start");
    for s in starts {
        let mut visited = visited.clone();
        if is_lower(s) {
            visited.insert(s.as_str());
        }
        res += recursive(s, graph, &mut visited);
    }
    res
}

fn main() -> std::io::Result<()> {
    let mut g = Graph(HashMap::new());

    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(Result::unwrap)
        .for_each(|s| {
            let (v, u) = s.split_once('-').unwrap();
            g.0.entry(v.into()).or_default().push(u.into());
            g.0.entry(u.into()).or_default().push(v.into());
        });

    let res = get_res(&g);
    println!("Result = {}", res);

    Ok(())
}
