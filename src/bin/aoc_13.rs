//! AoC 2021 - 13

use nalgebra as na;
use std::env;

#[derive(Debug, PartialEq, PartialOrd)]
enum Fold {
    X(i64),
    Y(i64),
}

fn parse_file(s: &str) -> (Vec<(i64, i64)>, Vec<Fold>) {
    let mut blank_split = s.split("\n\n");
    let dots_iter = blank_split.next().unwrap();
    let actions_iter = blank_split.next().unwrap();

    let dots = dots_iter
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut comma_split = l.split(',');
            let x = comma_split.next().unwrap().parse::<i64>().unwrap();
            let y = comma_split.next().unwrap().parse::<i64>().unwrap();
            (x, y)
        })
        .collect();

    let actions = actions_iter
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let equals_idx = l.find("=").unwrap();
            let fold_line = l[equals_idx + 1..].parse::<i64>().unwrap();
            match &l[equals_idx - 1..equals_idx] {
                "x" => Fold::X(fold_line),
                "y" => Fold::Y(fold_line),
                _ => unreachable!(),
            }
        })
        .collect();

    (dots, actions)
}

fn draw_dots(dots: &[(i64, i64)]) {
    let max_x = dots
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
        .0 as usize;
    let max_y = dots
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .1 as usize;
    let mut mat = na::DMatrix::<char>::repeat(max_y + 1, max_x + 1, '.');
    for (xc, yr) in dots {
        *mat.get_mut((*yr as usize, *xc as usize)).unwrap() = '#';
    }
    println!("{}", mat);
}

fn fold_dots(dots: &[(i64, i64)], fold: &Fold) -> Vec<(i64, i64)> {
    let mut new_dots = dots.to_vec();
    new_dots.sort();
    match fold {
        Fold::X(line) => {
            for (x, _y) in new_dots.iter_mut() {
                if *x > *line {
                    let dist = *x - *line;
                    *x = line - dist;
                }
            }
        }
        Fold::Y(line) => {
            for (_x, y) in new_dots.iter_mut() {
                if *y > *line {
                    let dist = *y - *line;
                    *y = line - dist;
                }
            }
        }
    };
    new_dots.sort();
    new_dots.dedup();
    new_dots
}

fn p1(dots: &[(i64, i64)], folds: &[Fold]) -> usize {
    let new_dots = fold_dots(dots, folds.first().unwrap());
    new_dots.len()
}

fn p2(dots: &[(i64, i64)], folds: &[Fold]) -> Vec<(i64, i64)> {
    let mut new_dots = dots.to_vec();
    for f in folds.iter() {
        new_dots = fold_dots(&new_dots, f);
    }
    new_dots
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (dots, folds) = parse_file(&file_str);
    let p1 = p1(&dots, &folds);
    println!("p1: {}", p1);
    let p2 = p2(&dots, &folds);
    println!("p2");
    draw_dots(&p2);
}

#[cfg(test)]
mod test_day13 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
        let (dots, folds) = parse_file(&example);
        eprintln!("{:?}", dots);
        eprintln!("{:?}", folds);
        let p1 = p1(&dots, &folds);
        assert_eq!(p1, 17);
    }
}
