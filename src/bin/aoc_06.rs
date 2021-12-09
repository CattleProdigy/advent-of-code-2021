//! AoC 2021 - 06

use std::env;

fn parse_file(s: &str) -> Vec<i64> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|s| s.parse::<i64>().unwrap()))
        .flatten()
        .collect::<Vec<_>>()
}

fn sim_fish(val: i64) -> (i64, bool) {
    let mut new_val = val;
    new_val -= 1;
    if new_val < 0 {
        (6, true)
    } else {
        (new_val, false)
    }
}

fn sim_fish_list_p1(fishes: &mut Vec<i64>, rounds: usize) {
    for _i in 0..rounds {
        let mut new_fishes = 0;
        for f in fishes.iter_mut() {
            let (new_val, new_fish) = sim_fish(*f);
            *f = new_val;
            if new_fish {
                new_fishes += 1;
            }
        }
        fishes.extend(std::iter::repeat(8).take(new_fishes));
    }
}

fn sim_fish_list_p2(fishes: &Vec<i64>, rounds: usize) -> usize {
    let mut fish_time_counts: Vec<usize> = vec![0; 10]; // last place is gutter for rotation
    for f in fishes {
        fish_time_counts[*f as usize] += 1;
    }
    for _i in 0..rounds {
        fish_time_counts.rotate_left(1);
        let rolled_counts = fish_time_counts[9];
        fish_time_counts[9] = 0;
        fish_time_counts[6] += rolled_counts;
        fish_time_counts[8] += rolled_counts;
    }

    fish_time_counts.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    {
        let mut fishes = parse_file(&file_str);
        sim_fish_list_p1(&mut fishes, 80);
        let p1_count = fishes.len();
        println!("p1: {}", p1_count);
    }
    {
        let mut fishes = parse_file(&file_str);
        let p2_count = sim_fish_list_p2(&mut fishes, 256);
        println!("p2: {}", p2_count);
    }
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    #[test]
    fn test() {
        let example = "3,4,3,1,2";
        {
            let mut fishes = parse_file(&example);
            let p1_count = sim_fish_list_p2(&mut fishes, 80);
            assert_eq!(p1_count, 5934);
        }
        {
            let mut fishes = parse_file(&example);
            let p2_count = sim_fish_list_p2(&mut fishes, 256);
            assert_eq!(p2_count, 26984457539);
        }
    }
}
