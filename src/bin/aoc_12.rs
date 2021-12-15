//! AoC 2021 - 12

use petgraph::graphmap::UnGraphMap;

use std::env;

fn parse_file(s: &str) -> UnGraphMap<&str, ()> {
    let mut graph = UnGraphMap::<&str, ()>::new();
    for l in s.lines().filter(|s| !s.is_empty()) {
        let mut it = l.split('-');
        let from = it.next().unwrap();
        let to = it.next().unwrap();
        graph.add_node(from);
        graph.add_node(to);
        graph.add_edge(from, to, ());
    }

    graph
}

fn p1p2(graph: &UnGraphMap<&str, ()>, p2: bool) -> usize {
    let mut traversal_stack = vec!["start"];
    let mut trajectory = vec![];
    let mut good_trajs = vec![];
    while let Some(cur) = traversal_stack.pop() {
        trajectory.push(cur);

        // Goal Node
        if cur == "end" {
            good_trajs.push(trajectory.iter().map(|s| s.to_string()).collect::<Vec<_>>());
            trajectory.pop();
            continue;
        } else if cur == "FINISH" {
            trajectory.pop();
            trajectory.pop();
            continue;
        }

        // Push neighbors
        traversal_stack.push("FINISH");
        for (_, to_node, _) in graph.edges(cur) {
            if p2 {
                if to_node == "start" {
                    continue;
                }

                if to_node.chars().all(char::is_lowercase)
                    && trajectory.iter().rev().find(|t| *t == &to_node).is_some()
                {
                    let mut uniq = std::collections::HashSet::new();
                    let already_has_dupe = trajectory
                        .iter()
                        .filter(|&x| x != &"start" && x.chars().all(char::is_lowercase))
                        .any(|&x| !uniq.insert(x));

                    if already_has_dupe {
                        continue;
                    }
                }
                traversal_stack.push(to_node);
            } else {
                if (to_node.chars().all(char::is_lowercase)
                    && trajectory.iter().rev().find(|t| *t == &to_node).is_some())
                    || to_node == "start"
                {
                    continue;
                }
                traversal_stack.push(to_node);
            }
        }
    }
    good_trajs.sort();
    for t in good_trajs.iter() {
        eprintln!("{:?}", t);
    }

    good_trajs.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let graph = parse_file(&file_str);
    println!("P1 {}", p1p2(&graph, false));
    println!("P2 {}", p1p2(&graph, true));
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
        let graph = parse_file(&example);
        eprintln!("graph: {:?}", graph);
        assert_eq!(p1p2(&graph, false), 10);
        assert_eq!(p1p2(&graph, true), 36);
    }
}
