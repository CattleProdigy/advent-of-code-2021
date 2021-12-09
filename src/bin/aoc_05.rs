//! AoC 2021 - 05

use std::{collections::HashMap, env};

fn parse_file(s: &str) -> Vec<((i64, i64), (i64, i64))> {
    s.lines()
        .map(|l| {
            let mut chunks = l.split_whitespace();
            let first_str = chunks.next().unwrap();
            let _skip = chunks.next().unwrap();
            let second_str = chunks.next().unwrap();

            let mut first_pair_itr = first_str.split(',');
            let first_x = first_pair_itr.next().unwrap().parse::<i64>().unwrap();
            let first_y = first_pair_itr.next().unwrap().parse::<i64>().unwrap();

            let mut second_pair_itr = second_str.split(',');
            let second_x = second_pair_itr.next().unwrap().parse::<i64>().unwrap();
            let second_y = second_pair_itr.next().unwrap().parse::<i64>().unwrap();
            ((first_x, first_y), (second_x, second_y))
        })
        .collect::<Vec<_>>()
}

fn map_vents(
    vents: &Vec<((i64, i64), (i64, i64))>,
    count_diag: bool,
) -> HashMap<(i64, i64), usize> {
    let mut map = HashMap::new();

    for &((x1, y1), (x2, y2)) in vents {
        if x1 == x2 {
            let mut range = [y1, y2];
            range.sort();
            for y in range[0]..=range[1] {
                *map.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let mut range = [x1, x2];
            range.sort();
            for x in range[0]..=range[1] {
                *map.entry((x, y1)).or_insert(0) += 1;
            }
        } else if count_diag {
            let x_slope = (x2 - x1).signum();
            let y_slope = (y2 - y1).signum();
            let dist = (y2 - y1).abs();

            for i in 0..=dist {
                let x = x1 + x_slope * i;
                let y = y1 + y_slope * i;
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    map
}

fn count_dangerous_vents(mapped: &HashMap<(i64, i64), usize>) -> usize {
    mapped.iter().filter(|((_x, _y), v)| **v >= 2).count()
}

fn run(s: &str) -> (usize, usize) {
    let vents = parse_file(&s);
    let p1_mapped = map_vents(&vents, false);
    let p1_count = count_dangerous_vents(&p1_mapped);
    println!("p1: {}", p1_count);
    let p2_mapped = map_vents(&vents, true);
    let p2_count = count_dangerous_vents(&p2_mapped);
    println!("p2: {}", p2_count);
    (p1_count, p2_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    run(&file_str);
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let (p1_count, p2_count) = run(&example);
        assert_eq!(p1_count, 5);
        assert_eq!(p2_count, 12);
    }
}
