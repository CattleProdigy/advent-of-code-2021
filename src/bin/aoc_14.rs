//! AoC 2021 - 14

use std::{collections::HashMap, convert::TryInto, env};

type RulesTable = HashMap<[char; 2], char>;
type FreqTable = HashMap<char, usize>;
type MemoTable = HashMap<([char; 2], usize), FreqTable>;

fn parse_file(s: &str) -> (Vec<char>, RulesTable) {
    let mut blank_split = s.split("\n\n");
    let template_iter = blank_split.next().unwrap();
    let rules_iter = blank_split.next().unwrap();

    let template = template_iter
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars())
        .flatten()
        .collect();

    let rules = rules_iter
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut delim = l.split(" -> ");
            let src = delim.next().unwrap();
            let src_arr = [src.chars().nth(0).unwrap(), src.chars().nth(1).unwrap()];

            let tgt = delim.next().unwrap().chars().nth(0).unwrap();
            (src_arr, tgt)
        })
        .collect();

    (template, rules)
}

fn step(poly: &[char], rules: &RulesTable) -> Vec<char> {
    let mut new_poly = Vec::new();
    for c in poly.windows(2) {
        let c_arr: [char; 2] = c.try_into().unwrap();
        new_poly.push(c[0]);

        new_poly.push(rules[&c_arr]);
    }
    new_poly.push(*poly.last().unwrap());

    new_poly
}

fn run_steps(poly: &[char], rules: &RulesTable, steps: usize) -> Vec<char> {
    let mut poly = poly.to_vec();
    for _i in 0..steps {
        poly = step(&poly, rules);
    }
    poly
}

fn p1(poly: &Vec<char>, rules: &RulesTable, steps: usize) -> usize {
    let result_poly = run_steps(poly, rules, steps);
    let mut freq = HashMap::new();
    for c in result_poly.iter() {
        *freq.entry(c).or_insert(0usize) += 1;
    }

    let most_common = freq
        .iter()
        .max_by(|(_, va), (_, vb)| va.partial_cmp(&vb).unwrap())
        .unwrap();
    let least_common = freq
        .iter()
        .min_by(|(_, va), (_, vb)| va.partial_cmp(&vb).unwrap())
        .unwrap();

    most_common.1 - least_common.1
}

fn p2_recur(
    poly: [char; 2],
    rules: &RulesTable,
    freq: &mut FreqTable,
    memo: &mut MemoTable,
    steps: usize,
    limit: usize,
) -> FreqTable {
    // No new additions
    if steps > limit {
        return FreqTable::new();
    }

    // If we've seen this "dimer" at this distance from the limit, then just return the cached
    // frequencies
    if let Some(table) = memo.get(&(poly, limit - steps)) {
        return table.clone();
    }

    // otherwise ...

    // get the new mer
    let new_mer = rules[&poly];

    // build the first new pair
    let first_pair = [poly[0], new_mer];
    // get the frequency contributions from that pair
    let mut result_table = p2_recur(first_pair, rules, freq, memo, steps + 1, limit);

    // build the first new pair
    let second_pair = [new_mer, poly[1]];
    // get the frequency contributions from that pair
    let second_table = p2_recur(second_pair, rules, freq, memo, steps + 1, limit);

    // merge the resulting ccontributions
    for (k, v) in second_table.iter() {
        *result_table.entry(*k).or_insert(0) += v;
    }
    // and add the new mer
    *result_table.entry(new_mer).or_insert(0) += 1;

    // cache it
    memo.insert((poly, limit - steps), result_table.clone());

    result_table
}

fn p2(poly: &[char], rules: &RulesTable, limit: usize) -> usize {
    let mut freq = HashMap::new();
    let mut memo = MemoTable::new();
    for c in poly.windows(2) {
        let c_arr: [char; 2] = c.try_into().unwrap();
        *freq.entry(c[0]).or_insert(0usize) += 1;

        let freq_table = p2_recur(c_arr, rules, &mut freq, &mut memo, 1, limit);
        for (k, v) in freq_table.iter() {
            *freq.entry(*k).or_insert(0) += v;
        }
    }
    *freq.entry(*poly.last().unwrap()).or_insert(0usize) += 1;

    let most_common = freq
        .iter()
        .max_by(|(_, va), (_, vb)| va.partial_cmp(&vb).unwrap())
        .unwrap();
    let least_common = freq
        .iter()
        .min_by(|(_, va), (_, vb)| va.partial_cmp(&vb).unwrap())
        .unwrap();

    most_common.1 - least_common.1
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("There should be one argument");
    }
    let file_str = std::fs::read_to_string(&args[1]).expect("couldn't read the file");
    let (template, rules) = parse_file(&file_str);
    println!("P1: {}", p1(&template, &rules, 10));
    println!("P2: {}", p2(&template, &rules, 40));
}

#[cfg(test)]
mod test_day14 {
    use super::*;

    #[test]
    fn test() {
        let example = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        let (template, rules) = parse_file(&example);
        eprintln!("{:?}", template);
        eprintln!("{:?}", rules);
        assert_eq!(p1(&template, &rules, 10), 1588);
        assert_eq!(p2(&template, &rules, 10), 1588);
        assert_eq!(p2(&template, &rules, 40), 2188189693529);
    }
}
