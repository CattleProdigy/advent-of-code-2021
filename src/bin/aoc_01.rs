//! AoC 2021 - 01

use std::env;

fn parse(s: &str) -> Vec<i64> {
    s.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn p1(depths: &[i64]) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(f, s)| f < s)
        .count()
}

fn p2(depths: &[i64]) -> usize {
    // Inefficient to call collect and call p1, but whatever
    p1(&depths
        .windows(3)
        .map(|slice| slice.iter().sum())
        .collect::<Vec<_>>())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");

    let depths = parse(&file_str);

    let p1_count = p1(&depths);
    println!("p1: {}", p1_count);
    let p2_count = p2(&depths);
    println!("p2: {}", p2_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let example = r#"199
200
208
210
200
207
240
269
260
263"#;

        let parsed = parse(example);
        assert_eq!(p1(&parsed), 7);
        assert_eq!(p2(&parsed), 5);
    }
}
