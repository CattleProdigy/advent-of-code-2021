//! AoC 2021 - 04

use std::{collections::HashMap, collections::HashSet, env};

static BINGOS: [[(usize, usize); 5]; 10/*12*/] = [
    //[(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)], // diag
    //[(4, 0), (3, 1), (2, 2), (1, 3), (0, 4)], // diag
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)], // cols
    [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)],
    [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
    [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)],
    [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)],
    [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)], // rows
    [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)],
    [(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)],
    [(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)],
    [(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)],
];

fn check_for_bingo(map: &HashSet<(usize, usize)>) -> bool {
    BINGOS.iter().any(|b| b.iter().all(|rc| map.contains(rc)))
}

fn score_board(board: &HashMap<usize, (usize, usize)>, marked: &HashSet<(usize, usize)>) -> usize {
    board
        .iter()
        .filter_map(|(v, (r, c))| {
            if !marked.contains(&(*r, *c)) {
                Some(*v)
            } else {
                None
            }
        })
        .sum()
}

fn parse_file(s: &str) -> (Vec<usize>, Vec<HashMap<usize, (usize, usize)>>) {
    let mut lines = s.lines().filter(|l| !l.is_empty());

    let called = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let remaining_lines = lines.collect::<Vec<_>>();

    let maps = remaining_lines
        .chunks(5)
        .map(|chunk| {
            let mut map = HashMap::new();
            for r in 0..5 {
                let line = chunk[r];
                for (c, x) in line.split_whitespace().enumerate() {
                    map.insert(x.parse::<usize>().unwrap(), (r, c));
                }
            }
            map
        })
        .collect::<Vec<_>>();

    (called, maps)
}

fn run_p1(called: &[usize], boards: &[HashMap<usize, (usize, usize)>]) -> Vec<(usize, usize)> {
    let mut board_progress = vec![HashSet::new(); boards.len()];
    for c in called {
        let mut winning_board_idxs = Vec::new();
        for (i, (b, bp)) in boards.iter().zip(board_progress.iter_mut()).enumerate() {
            if let Some((r, c)) = b.get(c) {
                bp.insert((*r, *c));
            }
            if check_for_bingo(bp) {
                let score = score_board(b, bp);
                let final_score = score * c;
                winning_board_idxs.push((i, final_score))
            }
        }
        if !winning_board_idxs.is_empty() {
            return winning_board_idxs;
        }
    }

    Vec::new()
}

fn run_p2(called: &[usize], boards: &[HashMap<usize, (usize, usize)>]) -> (usize, usize) {
    let mut board_progress = vec![HashSet::new(); boards.len()];
    let mut last = None;
    let mut bingo_mask = vec![false; boards.len()];
    for c in called {
        for (i, (b, bp)) in boards.iter().zip(board_progress.iter_mut()).enumerate() {
            if let Some((r, c)) = b.get(c) {
                bp.insert((*r, *c));
            }
            if check_for_bingo(bp) && !bingo_mask[i] {
                bingo_mask[i] = true;
                let score = score_board(b, bp);
                let final_score = score * c;
                last = Some((i, final_score));
            }
        }
    }

    last.unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (called, boards) = parse_file(&file_str);
    let p1_idxs_and_scores = run_p1(&called, &boards);
    println!("p1: {}", p1_idxs_and_scores.first().unwrap().1);
    let p2_idx_and_score = run_p2(&called, &boards);
    println!("p2: {}", p2_idx_and_score.1);
}

#[cfg(test)]
mod test_day4 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let (called, boards) = parse_file(example);
        let &(p1_i, p1_score) = run_p1(&called, &boards).first().unwrap();
        assert_eq!(p1_i, 2);
        assert_eq!(p1_score, 4512);
        let (p2_i, p2_score) = run_p2(&called, &boards);
        assert_eq!(p2_i, 1);
        assert_eq!(p2_score, 1924);
    }
}
