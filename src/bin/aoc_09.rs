//! AoC 2021 - 09

use nalgebra as na;
use std::{collections::VecDeque, env};

fn parse_file(s: &str) -> na::DMatrix<i64> {
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();
    let char_iter = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_string().parse::<i64>().unwrap()))
        .flatten();

    let src_mat = na::DMatrix::<i64>::from_iterator(cols, rows, char_iter).transpose();
    let mut padded_mat = na::DMatrix::<i64>::repeat(rows + 2, cols + 2, 9);
    padded_mat
        .slice_mut((1, 1), (rows, cols))
        .copy_from(&src_mat);
    padded_mat
}

fn p1(map: &na::DMatrix<i64>) -> (Vec<(usize, usize)>, i64) {
    let mut low_points = Vec::new();
    for r in 1..map.nrows() - 1 {
        for c in 1..map.ncols() - 1 {
            let cur = map[(r, c)];
            let up = *map.get((r - 1, c)).unwrap();
            let down = *map.get((r + 1, c)).unwrap();
            let left = *map.get((r, c - 1)).unwrap();
            let right = *map.get((r, c + 1)).unwrap();
            if cur < up && cur < down && cur < left && cur < right {
                low_points.push(((r, c), 1 + cur));
            }
        }
    }

    let risk = low_points.iter().map(|(_, risk)| risk).sum();
    let points = low_points.iter().map(|(rc, _risk)| *rc).collect::<Vec<_>>();
    (points, risk)
}

fn p2(map: &na::DMatrix<i64>, low_points: &[(usize, usize)]) -> Vec<usize> {
    let mut basins_assigments = na::DMatrix::<usize>::repeat(map.nrows(), map.ncols(), usize::MAX);
    let mut visited = na::DMatrix::<bool>::repeat(map.nrows(), map.ncols(), false);
    let mut traversal = VecDeque::<(usize, (usize, usize))>::new();

    for (i, &rc) in low_points.iter().enumerate() {
        traversal.push_front((i, (rc.0 as usize, rc.1 as usize)));
        basins_assigments[rc] = i;
    }

    while let Some((id, (r, c))) = traversal.pop_back() {
        if visited[(r, c)] {
            continue;
        }

        visited[(r, c)] = true;

        let cur = map[(r, c)];
        let up_rc = (r - 1, c);
        let up = *map.get(up_rc).unwrap();
        let down_rc = (r + 1, c);
        let down = *map.get(down_rc).unwrap();
        let left_rc = (r, c - 1);
        let left = *map.get(left_rc).unwrap();
        let right_rc = (r, c + 1);
        let right = *map.get(right_rc).unwrap();

        if up != 9 && up > cur {
            basins_assigments[up_rc] = id;
            traversal.push_back((id, up_rc));
        }
        if down != 9 && down > cur {
            basins_assigments[down_rc] = id;
            traversal.push_back((id, down_rc));
        }
        if left != 9 && left > cur {
            basins_assigments[left_rc] = id;
            traversal.push_back((id, left_rc));
        }
        if right != 9 && right > cur {
            basins_assigments[right_rc] = id;
            traversal.push_back((id, right_rc));
        }
    }

    let mut basin_counts = Vec::new();
    for i in basins_assigments.iter() {
        if *i == usize::MAX {
            continue;
        }
        if basin_counts.len() < *i + 1 {
            basin_counts.resize(i + 1, 0);
        }
        basin_counts[*i] += 1;
    }
    basin_counts
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let map = parse_file(&file_str);
    let (points, risk_sum) = p1(&map);
    println!("p1: {}", risk_sum);
    let mut basin_counts = p2(&map, &points);
    basin_counts.sort();
    let top3: usize = basin_counts.iter().rev().take(3).product();
    println!("p2: {}", top3);
}

#[cfg(test)]
mod test_day9 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        let map = parse_file(&example);
        eprintln!("{}", map);
        let (points, risk_sum) = p1(&map);
        assert_eq!(risk_sum, 15);
        let mut basin_counts = p2(&map, &points);
        basin_counts.sort();
        let top3: usize = basin_counts.iter().rev().take(3).product();
        assert_eq!(top3, 1134);
    }
}
