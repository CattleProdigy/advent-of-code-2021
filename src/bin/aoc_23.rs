//! AoC 2021 - 23

use petgraph::{
    algo::bellman_ford::*, algo::floyd_warshall::*, graph::NodeIndex, graph::UnGraph,
    visit::EdgeRef,
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy, Hash)]
struct Node {
    pub location: (usize, usize),
    pub occupied: char,
}

impl Node {
    fn new(location: (usize, usize), occupied: char) -> Self {
        Node { location, occupied }
    }
}

type Graph = UnGraph<Node, u64>;

fn print_graph(graph: &Graph) {
    let mut mat = nalgebra::DMatrix::<char>::repeat(5, 13, '#');
    for ni in graph.node_indices() {
        let node = graph[ni];
        *mat.get_mut((node.location.1 + 1, node.location.0 + 1))
            .unwrap() = node.occupied;
    }
    eprintln!("{}", mat);
}

const COST_MAX: u64 = 10000000;
fn movement_costs(
    graph: &Graph,
    src: &NodeIndex,
    tgt: Option<NodeIndex>,
) -> HashMap<NodeIndex, u64> {
    let edge_cost = match graph.node_weight(*src).unwrap().occupied {
        'A' => 1u64,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => {
            unreachable!()
        }
    };
    petgraph::algo::dijkstra(graph, *src, tgt, |edge| {
        if graph.node_weight(edge.target()).unwrap().occupied == '.' {
            edge_cost
        } else {
            COST_MAX
        }
    })
}

fn count_finished(graph: &Graph, map: &NodeMap) -> usize {
    let amph_nodes = graph
        .node_indices()
        .filter_map(|ni| {
            if graph.node_weight(ni).unwrap().occupied != '.' {
                Some(ni)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for an in amph_nodes {
        let node = graph[an];
        let in_hallway = node.location.1 == 0;
        if in_hallway {
            continue;
        }
        let targets = match node.occupied {
            'A' => map.a_nodes.to_vec(),
            'B' => map.b_nodes.to_vec(),
            'C' => map.c_nodes.to_vec(),
            'D' => map.d_nodes.to_vec(),
            _ => {
                unreachable!()
            }
        };
        let cleared_for_entry = targets.iter().all(|ni| {
            graph.node_weight(*ni).unwrap().occupied == '.'
                || graph.node_weight(*ni).unwrap().occupied == node.occupied
        });
        if cleared_for_entry {
            if targets.iter().find(|n| **n == an).is_some() {
                count += 1;
            }
        }
    }

    count
}

fn get_movement_opts(
    graph: &Graph,
    map: &NodeMap,
    cur_node_idx: &NodeIndex,
) -> Option<Vec<(NodeIndex, u64)>> {
    let node = graph.node_weight(*cur_node_idx).unwrap();
    assert!(node.occupied != '.');
    let in_hallway = node.location.1 == 0;
    let mut targets = match node.occupied {
        'A' => map.a_nodes.to_vec(),
        'B' => map.b_nodes.to_vec(),
        'C' => map.c_nodes.to_vec(),
        'D' => map.d_nodes.to_vec(),
        _ => {
            unreachable!()
        }
    };

    let in_place = targets
        .iter()
        .any(|t| graph.node_weight(*t).unwrap().location == node.location);

    let cleared_for_entry = targets.iter().all(|ni| {
        graph.node_weight(*ni).unwrap().occupied == '.'
            || graph.node_weight(*ni).unwrap().occupied == node.occupied
    });

    if cleared_for_entry && in_place {
        return None;
    }

    let mut max_y_target = None;
    if !cleared_for_entry {
        targets.clear();
    } else {
        max_y_target = targets
            .iter()
            .copied()
            .filter(|t| graph.node_weight(*t).unwrap().occupied == '.')
            .max_by(|&a, &b| {
                let a_node = graph.node_weight(a).unwrap();
                let b_node = graph.node_weight(b).unwrap();
                a_node.location.1.partial_cmp(&b_node.location.1).unwrap()
            });
    }
    //{
    //    eprintln!(
    //        "id: {:?}, src: {:?}, ih: {}, cfe: {}, myt: {:?}",
    //        cur_node_idx, node, in_hallway, cleared_for_entry, max_y_target
    //    );
    //}

    if in_hallway {
        if let Some(max_y_target) = max_y_target {
            // If we're in the hallway we can only go to a target
            let all_costs = movement_costs(graph, cur_node_idx, Some(max_y_target));
            //    eprintln!("hallwaly to target: {:?}", all_costs);
            let target_cost = all_costs[&max_y_target];
            if target_cost < COST_MAX || target_cost == 0 {
                Some(vec![(max_y_target, target_cost)])
            } else {
                Some(vec![])
            }
        } else {
            Some(vec![])
        }
    /*} else if !in_hallway && cleared_for_entry {
        // this node should already be done
        None
    */
    } else {
        // we're in some room to start
        let all_costs = movement_costs(graph, cur_node_idx, None);
        //eprintln!("{:?}", all_costs);
        let mut costs_filtered = vec![];
        for (ni, c) in all_costs.iter() {
            if *c >= COST_MAX || *c == 0 {
                continue;
            }
            if graph.node_weight(*ni).unwrap().occupied != '.' {
                continue;
            }
            if let Some(myt) = max_y_target {
                if myt == *ni {
                    costs_filtered.push((*ni, *c));
                }
            }
            if let Some(_) = map.avail_hall_nodes.iter().find(|x| **x == *ni) {
                costs_filtered.push((*ni, *c));
            }
        }
        //eprintln!("starting: {:?}", costs_filtered);
        Some(costs_filtered)
    }
}

fn p2_stack(graph: Graph, map: &NodeMap, memo: &mut HashMap<Vec<Node>, Option<u64>>) {
    let mut stack = Vec::<(u64, usize, Graph)>::new();
    let mut movement_options = vec![];

    {
        let amph_nodes = graph
            .node_indices()
            .filter_map(|ni| {
                if graph.node_weight(ni).unwrap().occupied != '.' {
                    Some(ni)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for an in amph_nodes.iter() {
            let mos = get_movement_opts(&graph, map, an);
            if mos.is_some() {
                for (ni, c) in mos.unwrap().iter().copied() {
                    movement_options.push((c, (*an, ni)));
                }
            };
        }

        for (cost, (from, to)) in movement_options {
            let mut new_graph = graph.clone();
            let old = new_graph.node_weight(from).unwrap().occupied;
            *(&mut new_graph.node_weight_mut(to).unwrap().occupied) = old;
            *(&mut new_graph.node_weight_mut(from).unwrap().occupied) = '.';
            stack.push((cost, 0, new_graph));
        }
    }

    let mut min_cost = u64::MAX;

    while let Some((cur_cost, num_finished, graph)) = stack.pop() {
        if cur_cost > min_cost {
            continue;
        }
        if num_finished >= 16 {
            if cur_cost < min_cost {
                min_cost = cur_cost.min(min_cost);
                eprintln!("solution with cost: {}", cur_cost);
            }
            continue;
        }
        let amph_nodes = graph
            .node_indices()
            .filter_map(|ni| {
                if graph.node_weight(ni).unwrap().occupied != '.' {
                    Some(ni)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut movement_options = vec![];
        for an in amph_nodes.iter() {
            let mos = get_movement_opts(&graph, map, an);
            if mos.is_some() {
                for (ni, c) in mos.unwrap().iter().copied() {
                    movement_options.push((c, (*an, ni)));
                }
            };
        }

        for (cost, (from, to)) in movement_options {
            let mut new_graph = graph.clone();
            let old = new_graph.node_weight(from).unwrap().occupied;
            *(&mut new_graph.node_weight_mut(to).unwrap().occupied) = old;
            *(&mut new_graph.node_weight_mut(from).unwrap().occupied) = '.';
            let count = count_finished(&new_graph, map);
            let mut memo_nodes = new_graph.node_weights().copied().collect::<Vec<_>>();
            memo_nodes.sort();

            if cur_cost + cost > min_cost {
                continue;
            }

            if let Some(cost_or_impossible) = memo.get(&memo_nodes) {
                if cost_or_impossible.is_none() {
                    // we've seen this configuration before and it's impossible
                    continue;
                } else {
                    let old_cost = cost_or_impossible.unwrap();

                    if cur_cost + cost >= old_cost {
                        // we've been here before and already doing worse so give up
                        continue;
                    } else {
                        *memo.get_mut(&memo_nodes).unwrap() = Some(cur_cost + cost);
                    }
                }
            } else {
                memo.insert(memo_nodes, Some(cur_cost + cost));
            }
            stack.push((cur_cost + cost, count, new_graph));
        }
    }

    // min_cost;
}

struct NodeMap {
    pub avail_hall_nodes: [NodeIndex; 11 - 4],
    pub a_nodes: [NodeIndex; 4],
    pub b_nodes: [NodeIndex; 4],
    pub c_nodes: [NodeIndex; 4],
    pub d_nodes: [NodeIndex; 4],
    //pub a_nodes: [NodeIndex; 2],
    //pub b_nodes: [NodeIndex; 2],
    //pub c_nodes: [NodeIndex; 2],
    //pub d_nodes: [NodeIndex; 2],
}

fn main() {
    let mut graph = Graph::new_undirected();

    let row_nodes = (0..11)
        .map(|x| graph.add_node(Node::new((x, 0), '.')))
        .collect::<Vec<_>>();
    for ns in row_nodes.windows(2) {
        graph.add_edge(ns[0], ns[1], 1);
    }

    // let a_col_nods = [
    //     graph.add_node(Node::new((2, 1), 'D')),
    //     graph.add_node(Node::new((2, 2), 'C')),
    // ];
    // graph.add_edge(a_col_nods[0], row_nodes[2], 1);
    // for ns in a_col_nods.windows(2) {
    //     graph.add_edge(ns[0], ns[1], 1);
    // }

    // let b_col_nods = [
    //     graph.add_node(Node::new((4, 1), 'B')),
    //     graph.add_node(Node::new((4, 2), 'C')),
    // ];
    // graph.add_edge(b_col_nods[0], row_nodes[4], 1);
    // for ns in b_col_nods.windows(2) {
    //     graph.add_edge(ns[0], ns[1], 1);
    // }

    // let c_col_nods = [
    //     graph.add_node(Node::new((6, 1), 'B')),
    //     graph.add_node(Node::new((6, 2), 'D')),
    // ];
    // graph.add_edge(c_col_nods[0], row_nodes[6], 1);
    // for ns in c_col_nods.windows(2) {
    //     graph.add_edge(ns[0], ns[1], 1);
    // }

    // let d_col_nods = [
    //     graph.add_node(Node::new((8, 1), 'A')),
    //     graph.add_node(Node::new((8, 2), 'A')),
    // ];

    let a_col_nods = [
        graph.add_node(Node::new((2, 1), 'D')),
        graph.add_node(Node::new((2, 2), 'D')),
        graph.add_node(Node::new((2, 3), 'D')),
        graph.add_node(Node::new((2, 4), 'C')),
    ];
    graph.add_edge(a_col_nods[0], row_nodes[2], 1);
    for ns in a_col_nods.windows(2) {
        graph.add_edge(ns[0], ns[1], 1);
    }

    let b_col_nods = [
        graph.add_node(Node::new((4, 1), 'B')),
        graph.add_node(Node::new((4, 2), 'C')),
        graph.add_node(Node::new((4, 3), 'B')),
        graph.add_node(Node::new((4, 4), 'C')),
    ];
    graph.add_edge(b_col_nods[0], row_nodes[4], 1);
    for ns in b_col_nods.windows(2) {
        graph.add_edge(ns[0], ns[1], 1);
    }

    let c_col_nods = [
        graph.add_node(Node::new((6, 1), 'B')),
        graph.add_node(Node::new((6, 2), 'B')),
        graph.add_node(Node::new((6, 3), 'A')),
        graph.add_node(Node::new((6, 4), 'D')),
    ];
    graph.add_edge(c_col_nods[0], row_nodes[6], 1);
    for ns in c_col_nods.windows(2) {
        graph.add_edge(ns[0], ns[1], 1);
    }

    let d_col_nods = [
        graph.add_node(Node::new((8, 1), 'A')),
        graph.add_node(Node::new((8, 2), 'A')),
        graph.add_node(Node::new((8, 3), 'C')),
        graph.add_node(Node::new((8, 4), 'A')),
    ];
    graph.add_edge(d_col_nods[0], row_nodes[8], 1);
    for ns in d_col_nods.windows(2) {
        graph.add_edge(ns[0], ns[1], 1);
    }
    let node_map = NodeMap {
        avail_hall_nodes: [
            row_nodes[0],
            row_nodes[1],
            row_nodes[3],
            row_nodes[5],
            row_nodes[7],
            row_nodes[9],
            row_nodes[10],
        ],
        a_nodes: a_col_nods,
        b_nodes: b_col_nods,
        c_nodes: c_col_nods,
        d_nodes: d_col_nods,
    };

    //eprintln!("{:?}", graph);
    let mut hashmap = HashMap::new();
    //p2(0, graph.clone(), &node_map, &mut hashmap);
    p2_stack(graph.clone(), &node_map, &mut hashmap);
}
