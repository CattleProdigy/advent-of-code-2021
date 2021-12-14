//! AoC 2021 - 11

use nalgebra as na;
use std::{collections::VecDeque, env};

const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    // [0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_file(s: &str) -> na::DMatrix<i64> {
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();
    let char_iter = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_string().parse::<i64>().unwrap()))
        .flatten();

    let src_mat = na::DMatrix::<i64>::from_iterator(cols, rows, char_iter).transpose();
    let mut padded_mat = na::DMatrix::<i64>::repeat(rows + 2, cols + 2, 10);
    padded_mat
        .slice_mut((1, 1), (rows, cols))
        .copy_from(&src_mat);
    padded_mat
}
fn sim_p1(map: &na::DMatrix<i64>) -> (na::DMatrix<i64>, usize) {
    let mut flashes = 0;
    let mut new_map = map.clone();

    // bump everything by 1
    for r in 1..map.nrows() - 1 {
        for c in 1..map.ncols() - 1 {
            *new_map.get_mut((r, c)).unwrap() += 1;
        }
    }

    // flash everything
    let mut queue = VecDeque::new();
    for r in 1..(map.nrows() - 1) {
        for c in 1..(map.ncols() - 1) {
            {
                let cur = new_map.get((r, c)).unwrap();
                if *cur == 10 {
                    queue.push_front((r, c))
                }
            }
        }
    }

    while let Some((rr, cc)) = queue.pop_back() {
        let cur = new_map.get_mut((rr, cc)).unwrap();

        // increment any flashes to 11+ to avoid loops, double flashes etc
        *cur += 1;
        flashes += 1;

        for (or, oc) in NEIGHBORS {
            let nr = ((rr as i64) + or) as usize;
            let nc = ((cc as i64) + oc) as usize;
            let neigh = new_map.get_mut((nr, nc)).unwrap();
            *neigh += 1;
            if *neigh == 10 && nr > 0 && nc > 0 && nr < (map.nrows() - 1) && nc < (map.ncols() - 1)
            {
                queue.push_front((nr, nc))
            }
        }
    }

    // reset zeros
    for r in 1..map.nrows() - 1 {
        for c in 1..map.ncols() - 1 {
            let cur = new_map.get_mut((r, c)).unwrap();
            if *cur >= 10 {
                *cur = 0
            }
        }
    }
    (new_map, flashes)
}

fn p1(map: &na::DMatrix<i64>, steps: usize) -> usize {
    let mut map = map.clone();
    let mut cum_sum = 0;
    for i in 0..steps {
        let (new_map, flashes) = sim_p1(&map);
        cum_sum += flashes;
        println!("After step {}: {}\n{}", i + 1, flashes, new_map);
        map = new_map;
    }
    eprintln!("{}", cum_sum);
    cum_sum
}

fn p2(map: &na::DMatrix<i64>) -> usize {
    let mut map = map.clone();
    let target_flashes = (map.ncols() - 2) * (map.nrows() - 2);
    let mut i = 1;
    loop {
        let (new_map, flashes) = sim_p1(&map);
        println!("After step {}: {}\n{}", i + 1, flashes, new_map);
        map = new_map;

        if flashes == target_flashes {
            eprintln!("{}", i);
            break;
        }
        i += 1;
    }
    i
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let map = parse_file(&file_str);
    p1(&map, 100);
    p2(&map);
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        let map = parse_file(&example);
        eprintln!("{}", map);
        p1(&map, 10);
        assert_eq!(p1(&map, 10), 204);
        assert_eq!(p1(&map, 100), 1656);
        assert_eq!(p2(&map), 195);
    }
}
