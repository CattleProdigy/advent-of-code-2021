//! AoC 2021 - 25

use nalgebra as na;
use std::env;

fn parse_file(s: &str) -> na::DMatrix<char> {
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();
    let char_iter = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars())
        .flatten();

    let src_mat = na::DMatrix::<char>::from_iterator(cols, rows, char_iter).transpose();
    src_mat
}

fn step(map: &mut na::DMatrix<char>) -> usize {
    let r = map.nrows();
    let c = map.ncols();
    let mut movement_count = 0;

    let mut east_steps = vec![];
    for cc in 0..c {
        for rr in 0..r {
            let cur = map.get_mut((rr, cc)).unwrap();
            if *cur != '>' {
                continue;
            }
            let neigh_loc_c = (cc + 1) % c;
            let neigh = map.get((rr, neigh_loc_c)).unwrap();
            if *neigh == '.' {
                east_steps.push((rr, cc));
            }
        }
    }
    movement_count += east_steps.len();
    for (rr, cc) in east_steps {
        let neigh_loc_c = (cc + 1) % c;
        *map.get_mut((rr, cc)).unwrap() = '.';
        *map.get_mut((rr, neigh_loc_c)).unwrap() = '>';
    }

    let mut south_steps = vec![];
    for cc in 0..c {
        for rr in 0..r {
            let cur = map.get_mut((rr, cc)).unwrap();
            if *cur != 'v' {
                continue;
            }
            let neigh_loc_r = (rr + 1) % r;
            let neigh = map.get((neigh_loc_r, cc)).unwrap();
            if *neigh == '.' {
                south_steps.push((rr, cc));
            }
        }
    }
    movement_count += south_steps.len();
    for (rr, cc) in south_steps {
        let neigh_loc_r = (rr + 1) % r;
        *map.get_mut((rr, cc)).unwrap() = '.';
        *map.get_mut((neigh_loc_r, cc)).unwrap() = 'v';
    }

    movement_count
}

fn run_to_steady(map: na::DMatrix<char>) -> usize {
    let mut map = map;

    let mut count = 0;
    loop {
        let movement_count = step(&mut map);
        count += 1;
        if movement_count == 0 {
            break;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let map = parse_file(&file_str);
    let p1 = run_to_steady(map.clone());
    println!("P1: {}", p1);
}

#[cfg(test)]
mod test_day25 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
        let map = parse_file(&example);
        eprintln!("{}", map);
        assert_eq!(run_to_steady(map.clone()), 58);
    }
}
