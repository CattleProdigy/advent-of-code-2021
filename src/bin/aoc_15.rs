//! AoC 2021 - 15

use nalgebra as na;
use std::{cmp::*, collections::BinaryHeap, env};

fn parse_file(s: &str) -> na::DMatrix<i64> {
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();
    let char_iter = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_string().parse::<i64>().unwrap()))
        .flatten();

    na::DMatrix::<i64>::from_iterator(cols, rows, char_iter).transpose()
}

#[derive(PartialEq, Eq, Debug)]
struct SearchNode {
    heur_cost: i64,
    location: (usize, usize),
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.heur_cost.cmp(&other.heur_cost).reverse())
    }
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heur_cost.cmp(&other.heur_cost).reverse()
    }
}

fn replicate_map(map: &na::DMatrix<i64>, times: usize) -> na::DMatrix<i64> {
    let rows = map.nrows();
    let cols = map.ncols();
    let new_rows = rows * times;
    let new_cols = rows * times;

    let mut new_map = na::DMatrix::<i64>::zeros(new_rows, new_cols);
    new_map.slice_mut((0, 0), (rows, cols)).copy_from(&map);
    for c in 1..times {
        let mat_left_col = (c - 1) * cols;
        let mut src_mat = new_map.slice((0, mat_left_col), (rows, cols)).into_owned();
        for v in src_mat.iter_mut() {
            *v = if *v >= 9 { 1 } else { *v + 1 };
        }
        new_map
            .slice_mut((0, c * cols), (rows, cols))
            .copy_from(&src_mat);
    }
    for r in 1..times {
        let row_above = (r - 1) * rows;
        let row = (r) * rows;
        for c in 0..times {
            let col = c * cols;
            let mut src_mat = new_map.slice((row_above, col), (rows, cols)).into_owned();
            for v in src_mat.iter_mut() {
                *v = if *v >= 9 { 1 } else { *v + 1 };
            }
            new_map
                .slice_mut((row, col), (rows, cols))
                .copy_from(&src_mat);
        }
    }

    new_map
}

fn astar(mat: &na::DMatrix<i64>) -> i64 {
    let mut open_set = BinaryHeap::<SearchNode>::new();
    let mut from_map =
        na::DMatrix::<(usize, usize)>::repeat(mat.nrows(), mat.ncols(), (usize::MAX, usize::MAX));
    let mut cost_map = na::DMatrix::<i64>::repeat(mat.nrows(), mat.ncols(), i64::MAX);
    *cost_map.get_mut((0, 0)).unwrap() = 0;

    open_set.push(SearchNode {
        heur_cost: mat[(0, 0)],
        location: (0, 0),
    });

    let rows = mat.nrows();
    let cols = mat.nrows();
    let target = (rows - 1, cols - 1);

    while let Some(SearchNode {
        heur_cost: _cur_heur_cost,
        location: cur_loc,
    }) = open_set.pop()
    {
        if cur_loc == target {
            break;
        }

        let cur_cost = cost_map[cur_loc];

        const NEIGH_OFFSETS: [(i64, i64); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

        for (or, oc) in NEIGH_OFFSETS {
            let nr = or + cur_loc.0 as i64;
            let nc = oc + cur_loc.1 as i64;

            if nr < 0 || nr >= rows as i64 || nc < 0 || nc >= cols as i64 {
                continue;
            }

            let neigh = (nr as usize, nc as usize);
            let estimated_cost = cur_cost + mat[neigh];
            if estimated_cost < cost_map[neigh] {
                from_map[neigh] = cur_loc;
                cost_map[neigh] = estimated_cost;
                let heuristic = 0; //(((rows - neigh.0) + (cols - neigh.1)) * 10) as i64;
                open_set.push(SearchNode {
                    heur_cost: estimated_cost + heuristic,
                    location: neigh,
                });
            }
        }
    }

    eprintln!("map: {}", cost_map[target]);

    let mut traj = Vec::new();
    let mut cur = target;
    while cur != (0, 0) {
        traj.push(cur);
        cur = from_map[cur];
    }
    traj.push((0, 0));
    traj.reverse();
    eprintln!("traj:{:?}", traj);

    let vals = traj.iter().map(|rc| mat[*rc]).collect::<Vec<_>>();
    eprintln!("vals:{:?}", vals);

    cost_map[target]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let map = parse_file(&file_str);
    let p1_total = astar(&map);
    let big_map = replicate_map(&map, 5);
    let p2_total = astar(&big_map);
    println!("P1: {}", p1_total);
    println!("P2: {}", p2_total);
}

#[cfg(test)]
mod test_day15 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

        let map = parse_file(&example);
        let p1_total = astar(&map);
        let big_map = replicate_map(&map, 5);
        let p2_total = astar(&big_map);

        assert_eq!(p1_total, 40);
        assert_eq!(p2_total, 315);
    }
}
