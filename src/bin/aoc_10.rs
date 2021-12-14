//! AoC 2021 - 10

use std::{collections::VecDeque, env};

enum LineResult {
    Incomplete(Vec<char>),
    Illegal(char),
    Complete,
}

fn matched(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        _ => unreachable!("unknown char"),
    }
}

fn p1p2(s: &str) -> (usize, usize) {
    let mut results = Vec::new();
    for l in s.lines() {
        let mut stack = VecDeque::new();
        let mut should_break = false;
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                }
                ')' | ']' | '}' | '>' => {
                    if let Some(o) = stack.pop_back() {
                        if o == matched(c) {
                            // println!("completed: ()");
                        } else {
                            println!("expect '{}', but found '{}'", matched(c), c);
                            results.push(LineResult::Illegal(c));
                            should_break = true;
                        }
                    }
                }
                _ => {
                    unreachable!("unknown char: '{}'", c);
                }
            }
        }
        if should_break {
            continue;
        }
        if !stack.is_empty() {
            let mut completion = Vec::new();
            while let Some(o) = stack.pop_back() {
                completion.push(matched(o));
            }
            results.push(LineResult::Incomplete(completion));
        } else {
            results.push(LineResult::Complete);
        }
    }

    let score_p1: usize = results
        .iter()
        .map(|r| match r {
            LineResult::Illegal(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
            _ => 0,
        })
        .sum();
    let mut p2_line_scores = results
        .iter()
        .filter_map(|r| match r {
            LineResult::Incomplete(c) => {
                let ls = c.iter().fold(0, |acc, x| {
                    acc * 5
                        + match x {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                });
                Some(ls)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    p2_line_scores.sort();
    let median = p2_line_scores[p2_line_scores.len() / 2];

    (score_p1, median)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (p1_score, p2_score) = p1p2(&file_str);
    println!("p1: {}", p1_score);
    println!("p2: {}", p2_score);
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let (p1_score, p2_score) = p1p2(&example);
        assert_eq!(p1_score, 26397);
        assert_eq!(p2_score, 288957);
    }
}
