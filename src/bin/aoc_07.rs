//! AoC 2021 - 07

use std::env;

fn parse_file(s: &str) -> Vec<i64> {
    let mut v = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|s| s.parse::<i64>().unwrap()))
        .flatten()
        .collect::<Vec<_>>();
    v.sort();
    v
}

fn cum_sum(x: i64) -> i64 {
    x * (x + 1) / 2
}

fn p2(crabs: &[i64]) -> (i64, i64) {
    let &first_pos = crabs.first().unwrap();
    let &last_pos = crabs.last().unwrap();

    let mut winning_pos = 0;
    let mut winning_score = i64::MAX;
    for p in first_pos..=last_pos {
        let score = crabs.iter().map(|c| cum_sum((c - p).abs())).sum::<i64>();
        if score < winning_score {
            winning_score = score;
            winning_pos = p;
        }
    }
    assert!(winning_score != i64::MAX);

    (winning_pos, winning_score)
}

fn p1(crabs: &[i64]) -> (i64, i64) {
    let &first_pos = crabs.first().unwrap();
    let &last_pos = crabs.last().unwrap();

    let mut winning_pos = 0;
    let mut winning_score = i64::MAX;
    for p in first_pos..=last_pos {
        let score = crabs.iter().map(|c| (c - p).abs()).sum::<i64>();
        if score < winning_score {
            winning_score = score;
            winning_pos = p;
        }
    }
    assert!(winning_score != i64::MAX);

    (winning_pos, winning_score)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let mut crabs = parse_file(&file_str);
    let (pos, score) = p1(&mut crabs);
    println!("P1: pos: {}, fuel: {}", pos, score);
    let (p2_pos, p2_score) = p2(&mut crabs);
    println!("P2: pos: {}, fuel: {}", p2_pos, p2_score);
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    #[test]
    fn test() {
        let example = "16,1,2,0,4,2,7,1,2,14";
        {
            let mut crabs = parse_file(&example);
            let (pos, score) = p1(&mut crabs);
            assert_eq!(pos, 2);
            assert_eq!(score, 37);
            let (p2_pos, p2_score) = p2(&mut crabs);
            assert_eq!(p2_pos, 5);
            assert_eq!(p2_score, 168);
        }
    }
}
