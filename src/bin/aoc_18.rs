//! AoC 2021 - 18

use std::env;

#[derive(Debug, Eq, PartialEq, Clone)]
enum SnailNumInner {
    SnailNum(Box<SnailNum>),
    Number(i64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SnailNum((SnailNumInner, SnailNumInner));

fn propagate_val_left(s: &mut SnailNum, val: i64) -> bool {
    match &mut s.0 .1 {
        SnailNumInner::Number(n) => {
            *n += val;
            return true;
        }
        SnailNumInner::SnailNum(sn) => {
            let propagated = propagate_val_left(sn, val);
            if propagated {
                return propagated;
            }
        }
    }
    match &mut s.0 .0 {
        SnailNumInner::Number(n) => {
            *n += val;
            return true;
        }
        SnailNumInner::SnailNum(sn) => {
            return propagate_val_left(sn, val);
        }
    }
}

fn propagate_val_right(s: &mut SnailNum, val: i64) -> bool {
    match &mut s.0 .0 {
        SnailNumInner::Number(n) => {
            *n += val;
            return true;
        }
        SnailNumInner::SnailNum(sn) => {
            let propagated = propagate_val_right(sn, val);
            if propagated {
                return propagated;
            }
        }
    }
    match &mut s.0 .1 {
        SnailNumInner::Number(n) => {
            *n += val;
            return true;
        }
        SnailNumInner::SnailNum(sn) => {
            return propagate_val_right(sn, val);
        }
    }
}

fn explode_recurse(sn: &mut SnailNum, count: usize) -> (bool, bool, Option<i64>, Option<i64>) {
    if count >= 4 {
        let left = match sn.0 .0 {
            SnailNumInner::Number(num) => num,
            _ => panic!("should be a num"),
        };
        let right = match sn.0 .1 {
            SnailNumInner::Number(num) => num,
            _ => panic!("should be a num"),
        };
        return (true, true, Some(left), Some(right));
    } else {
        match &mut sn.0 .0 {
            // first .0 is to get passed the newtype, second is for the left pair
            SnailNumInner::SnailNum(sn_inner) => {
                let (anything_changed, becomes_zero, new_l, mut new_r) =
                    explode_recurse(&mut *sn_inner, count + 1);
                if becomes_zero {
                    sn.0 .0 = SnailNumInner::Number(0);
                }
                if let Some(new_r_inner) = new_r {
                    match &mut sn.0 .1 {
                        SnailNumInner::Number(num) => {
                            *num += new_r_inner;
                            new_r = None;
                        }
                        SnailNumInner::SnailNum(sn_inner) => {
                            if propagate_val_right(sn_inner, new_r_inner) {
                                new_r = None;
                            }
                        }
                    }
                }
                if anything_changed {
                    return (true, false, new_l, new_r);
                }
            }
            _ => {}
        }
        match &mut sn.0 .1 {
            SnailNumInner::SnailNum(sn_inner) => {
                let (anything_changed, becomes_zero, mut new_l, new_r) =
                    explode_recurse(&mut *sn_inner, count + 1);
                if becomes_zero {
                    sn.0 .1 = SnailNumInner::Number(0);
                }
                if let Some(new_l_inner) = new_l {
                    match &mut sn.0 .0 {
                        SnailNumInner::Number(num) => {
                            *num += new_l_inner;
                            new_l = None;
                        }
                        SnailNumInner::SnailNum(sn_inner) => {
                            if propagate_val_left(sn_inner, new_l_inner) {
                                new_l = None;
                            }
                        }
                    }
                }
                if anything_changed {
                    return (true, false, new_l, new_r);
                }
            }
            _ => {}
        }
    }

    return (false, false, None, None);
}

fn explode_til_completion(sn: &SnailNum) -> SnailNum {
    let mut res = sn.clone();
    while explode_recurse(&mut res, 0).0 {}
    res
}

fn split_once(sn: &mut SnailNum) -> bool {
    match &mut sn.0 .0 {
        SnailNumInner::Number(num) => {
            if *num >= 10 {
                let l = *num / 2;
                let r = (*num + 1) / 2;

                sn.0 .0 = SnailNumInner::SnailNum(Box::new(SnailNum((
                    SnailNumInner::Number(l),
                    SnailNumInner::Number(r),
                ))));

                return true;
            }
        }
        SnailNumInner::SnailNum(sn_inner) => {
            if split_once(sn_inner) {
                return true;
            }
        }
    };
    match &mut sn.0 .1 {
        SnailNumInner::Number(num) => {
            if *num >= 10 {
                let l = *num / 2;
                let r = (*num + 1) / 2;

                sn.0 .1 = SnailNumInner::SnailNum(Box::new(SnailNum((
                    SnailNumInner::Number(l),
                    SnailNumInner::Number(r),
                ))));

                return true;
            }
        }
        SnailNumInner::SnailNum(sn_inner) => {
            return split_once(sn_inner);
        }
    };

    return false;
}

fn reduce(sn: &SnailNum) -> SnailNum {
    let mut res = sn.clone();
    let mut last = None;
    while last.is_none() || last.unwrap() != res {
        last = Some(res.clone());
        res = explode_til_completion(&res);
        split_once(&mut res);
    }

    res
}

fn add(lhs: &SnailNum, rhs: &SnailNum) -> SnailNum {
    let added = SnailNum((
        SnailNumInner::SnailNum(Box::new(lhs.clone())),
        SnailNumInner::SnailNum(Box::new(rhs.clone())),
    ));

    reduce(&added)
}

fn parse_all(s: &str) -> Vec<SnailNum> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse(l).1)
        .collect()
}

fn parse(s: &str) -> (usize, SnailNum) {
    assert!(s.is_ascii());
    assert_eq!(&s[0..1], "[");
    let mut reslice = &s[1..];
    let mut idx = 1;

    // First
    let first_inner = if &reslice[0..1] == "[" {
        let (step, num) = parse(&reslice);
        reslice = &reslice[step + 1..];
        idx += step + 1;
        SnailNumInner::SnailNum(Box::new(num))
    } else {
        let comma_idx = &reslice.find(',').unwrap();
        let num = &reslice[0..*comma_idx].parse::<i64>().unwrap();
        reslice = &reslice[comma_idx + 1..];
        idx += comma_idx + 1;
        SnailNumInner::Number(*num)
    };

    // Second
    let second_inner = if &reslice[0..1] == "[" {
        let (step, num) = parse(&reslice);
        //reslice = &reslice[step + 1..];
        idx += step + 1;
        SnailNumInner::SnailNum(Box::new(num))
    } else {
        let bracket_idx = &reslice.find(']').unwrap();
        let num = &reslice[0..*bracket_idx].parse::<i64>().unwrap();
        //reslice = &reslice[bracket_idx + 1..];
        idx += bracket_idx + 1;
        SnailNumInner::Number(*num)
    };

    (idx, SnailNum((first_inner, second_inner)))
}

fn mag_recurse(s: &SnailNum) -> i64 {
    let left = match &s.0 .0 {
        SnailNumInner::Number(n) => *n,
        SnailNumInner::SnailNum(sn) => mag_recurse(&sn),
    };
    let right = match &s.0 .1 {
        SnailNumInner::Number(n) => *n,
        SnailNumInner::SnailNum(sn) => mag_recurse(&sn),
    };
    3 * left + 2 * right
}

fn p1(nums: &[SnailNum]) -> i64 {
    let sum = nums
        .iter()
        .skip(1)
        .fold(nums[0].clone(), |acc, x| add(&acc, x));
    mag_recurse(&sum)
}

fn p2(nums: &[SnailNum]) -> i64 {
    let mut mag_max = i64::MIN;

    for i in 0..nums.len() {
        let i_num = &nums[i];
        for j in i + 1..nums.len() {
            let j_num = &nums[j];
            let ij_arr = [i_num.clone(), j_num.clone()];
            let ij_mag = p1(&ij_arr[..]);
            let ji_arr = [j_num.clone(), i_num.clone()];
            let ji_mag = p1(&ji_arr);
            mag_max = mag_max.max(ij_mag);
            mag_max = mag_max.max(ji_mag);
        }
    }

    mag_max
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let nums = parse_all(&file_str);
    let p1_res = p1(&nums);
    println!("P1: {}", p1_res);
    let p2_res = p2(&nums);
    println!("P2: {}", p2_res);
}

#[cfg(test)]
mod test_day18 {
    use super::*;

    #[test]
    fn test() {
        {
            let ex = parse("[[[[[9,8],1],2],3],4]").1;
            let answer = parse("[[[[0,9],2],3],4]").1;

            let mut res = ex.clone();
            let (changed, zero, l, r) = explode_recurse(&mut res, 0);
            assert!(changed);
            assert!(!zero);
            assert!(l.is_some());
            assert!(r.is_none());
            assert_eq!(res, answer);
        }
        {
            let ex = parse("[7,[6,[5,[4,[3,2]]]]]").1;
            let answer = parse("[7,[6,[5,[7,0]]]]").1;

            let mut res = ex.clone();
            let (changed, zero, l, r) = explode_recurse(&mut res, 0);
            assert!(changed);
            assert!(!zero);
            assert!(l.is_none());
            assert!(r.is_some());
            assert_eq!(res, answer);
        }
        {
            let ex = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").1;
            let answer = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").1;

            let mut res = ex.clone();
            let (changed, zero, l, r) = explode_recurse(&mut res, 0);
            assert!(changed);
            assert!(!zero);
            assert!(l.is_none());
            assert!(r.is_none());
            assert_eq!(res, answer);
        }
        {
            let lhs = parse("[[[[4,3],4],4],[7,[[8,4],9]]]").1;
            let rhs = parse("[1,1]").1;
            let answer = parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").1;

            let added = add(&lhs, &rhs);
            assert_eq!(added, answer);
        }
    }
}
