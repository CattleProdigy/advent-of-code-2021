//! AoC 2021 - 20

use nalgebra as na;
use std::env;

fn parse(s: &str) -> (Vec<i64>, na::DMatrix<i64>) {
    let mut linebreak = s.split("\n\n");
    let first_line = linebreak
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();

    let remainder = linebreak.next().unwrap();

    let rows = remainder.lines().count();
    let cols = remainder.lines().next().unwrap().len();
    let char_iter = remainder
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }))
        .flatten();

    let src_mat = na::DMatrix::<i64>::from_iterator(cols, rows, char_iter).transpose();
    (first_line, src_mat)
}

fn process(lut: &Vec<i64>, mat: &na::DMatrix<i64>) -> na::DMatrix<i64> {
    let rows = mat.nrows();
    let cols = mat.ncols();
    let mut new_mat = na::DMatrix::<i64>::zeros(rows, cols);

    for c in 1..(cols - 1) {
        for r in 1..(rows - 1) {
            let mut val = 0;
            val += mat[(r - 1, c - 1)] << 8;
            val += mat[(r - 1, c)] << 7;
            val += mat[(r - 1, c + 1)] << 6;
            val += mat[(r, c - 1)] << 5;
            val += mat[(r, c)] << 4;
            val += mat[(r, c + 1)] << 3;
            val += mat[(r + 1, c - 1)] << 2;
            val += mat[(r + 1, c)] << 1;
            val += mat[(r + 1, c + 1)] << 0;
            *new_mat.get_mut((r, c)).unwrap() = lut[val as usize];
        }
    }
    new_mat
}

fn p1(lut: &Vec<i64>, mat: &na::DMatrix<i64>) -> (na::DMatrix<i64>, i64) {
    let rows = mat.nrows();
    let cols = mat.ncols();
    // adding two values on each edge, 1 val is for look up another for writing
    let mut padded_mat = na::DMatrix::<i64>::zeros(rows + 4, cols + 4);
    padded_mat.slice_mut((2, 2), (rows, cols)).copy_from(&mat);
    padded_mat = process(lut, &padded_mat);
    let mut padded_mat2 =
        na::DMatrix::<i64>::repeat(padded_mat.nrows() + 4, padded_mat.ncols() + 4, lut[0]);
    padded_mat2
        .slice_mut((2, 2), (padded_mat.nrows() - 2, padded_mat.ncols() - 2))
        .copy_from(&padded_mat.slice_mut((1, 1), (padded_mat.nrows() - 2, padded_mat.ncols() - 2)));
    padded_mat2 = process(lut, &padded_mat2);
    let sum = padded_mat2.sum();
    (padded_mat2, sum)
}

fn p2(lut: &Vec<i64>, mat: &na::DMatrix<i64>) -> i64 {
    let mut mat = mat.clone();
    for _i in 0..25 {
        let (new_pm, _sum) = p1(lut, &mat);
        mat = new_pm;
    }

    mat.sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (lut, mat) = parse(&file_str);
    let (_, p1) = p1(&lut, &mat);
    eprintln!("p1: {}", p1);
    let p2 = p2(&lut, &mat);
    eprintln!("p2: {}", p2);
}

#[cfg(test)]
mod test_day20 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

        let (lut, mat) = parse(&example);
        let (_, p1) = p1(&lut, &mat);
        assert_eq!(p1, 35);
        let p2 = p2(&lut, &mat);
        assert_eq!(p2, 3351);
    }
}
