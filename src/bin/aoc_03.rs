//! AoC 2021 - 03

use std::env;

fn parse_file(s: &str) -> Vec<u16> {
    s.lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

fn most_common_bit_pattern(nums: &[u16], bit_depth: usize, tie: u16) -> u16 {
    let mut result_bits = vec![0; bit_depth];
    for i in 0..bit_depth {
        let mut diff_count = 0;
        for n in nums {
            let n_shift = n >> i;
            if n_shift & 0x1 == 0 {
                diff_count -= 1;
            } else {
                diff_count += 1;
            }
        }
        result_bits[bit_depth - 1 - i] = if diff_count < 0 {
            0
        } else if diff_count > 0 {
            1
        } else {
            tie
        };
    }

    u16::from_str_radix(
        &result_bits
            .iter()
            .map(|x| x.to_string())
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            }),
        2,
    )
    .unwrap()
}

fn p1(nums: &[u16], bit_depth: usize) -> (u64, u64) {
    let gamma = most_common_bit_pattern(nums, bit_depth, 0);

    // Negate but zero out the hiigher order bits
    let epsilon = (!gamma << (16 - bit_depth)) >> (16 - bit_depth);

    (gamma as u64, epsilon as u64)
}

fn p2(nums: &[u16], bit_depth: usize) -> (u64, u64) {
    let mut o2_nums = nums.to_vec();
    o2_nums.sort();
    let mut co2_nums = o2_nums.clone();
    for i in (0..bit_depth).rev() {
        let mut diff_count = 0;
        for o2 in o2_nums.iter() {
            if (o2 >> i) & 0x1 == 0 {
                diff_count -= 1;
            } else {
                diff_count += 1;
            }
        }
        let o2_matcher = if diff_count >= 0 { 1 } else { 0 };
        if o2_nums.len() > 1 {
            o2_nums.retain(|n| {
                let bit = (n >> i) & 0x1;
                bit == o2_matcher
            });
        }

        let mut diff_count = 0;
        for co2 in co2_nums.iter() {
            if (co2 >> i) & 0x1 == 0 {
                diff_count -= 1;
            } else {
                diff_count += 1;
            }
        }
        let co2_matcher = if diff_count >= 0 { 0 } else { 1 };
        if co2_nums.len() > 1 {
            co2_nums.retain(|n| {
                let bit = (n >> i) & 0x1;
                bit == co2_matcher
            });
        }
    }
    if o2_nums.len() != 1 {
        panic!("[o2] we went through our bits without narrowing down to one");
    }
    if co2_nums.len() != 1 {
        panic!("[o2] we went through our bits without narrowing down to one");
    }

    (o2_nums[0] as u64, co2_nums[0] as u64)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");

    let nums = parse_file(&file_str);
    let (p1_gam, p1_eps) = p1(&nums, 12);
    let p1_prod = p1_gam * p1_eps;
    println!("p1: {}", p1_prod);
    let (p2_gam, p2_eps) = p2(&nums, 12);
    let p2_prod = p2_gam * p2_eps;
    println!("p2: {}", p2_prod);
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

        let nums = parse_file(example);
        let (gam, eps) = p1(&nums, 5);
        assert_eq!(gam, 22);
        assert_eq!(eps, 9);
        let (o2, co2) = p2(&nums, 5);
        assert_eq!(o2, 23);
        assert_eq!(co2, 10);
    }
}
