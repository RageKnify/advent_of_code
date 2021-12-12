use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Graph(HashMap<String, Vec<String>>);

fn is_lower(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn get_res(graph: &Graph) -> Vec<Vec<String>> {
    fn recursive<'graph, 's: 'graph>(
        curr: &'s str,
        graph: &'graph Graph,
        visited: &mut HashSet<&'graph str>,
        double_lower: Option<&str>,
        path: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let next_ones = &graph.0[curr];
        for n in next_ones {
            if n == "start" || visited.contains(n.as_str()) {
                continue;
            }
            let mut path = path.clone();
            path.push(n.into());
            if n == "end" {
                res.push(path.clone());
                continue;
            }
            let mut visited = visited.clone();
            if is_lower(n) {
                if double_lower.is_none() {
                    res.extend_from_slice(&recursive(
                        n,
                        graph,
                        &mut visited.clone(),
                        Some(n),
                        path.clone(),
                    ));
                }
                visited.insert(n);
            }
            res.extend_from_slice(&recursive(
                n,
                graph,
                &mut visited.clone(),
                double_lower,
                path,
            ));
        }
        res
    }

    let mut res = Vec::new();
    let mut path = Vec::new();
    let starts = &graph.0["start"];
    path.push("start".into());
    let mut visited = HashSet::new();
    visited.insert("start");
    for s in starts {
        let mut path = path.clone();
        path.push(s.into());
        let mut visited = visited.clone();
        if is_lower(s) {
            res.extend_from_slice(&recursive(
                s,
                graph,
                &mut visited.clone(),
                Some(s),
                path.clone(),
            ));
            visited.insert(s.as_str());
        }
        res.extend_from_slice(&recursive(s, graph, &mut visited, None, path.clone()));
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

    let mut res = get_res(&g);
    res.sort();
    res.dedup();
    println!("Result = {}", res.len());

    Ok(())
}
