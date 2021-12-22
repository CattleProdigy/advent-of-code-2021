//! AoC 2021 - 21

use std::collections::HashMap;

fn p1_score_at_n_turns(start: i64, n: i64) -> i64 {
    const REM_TABLE: [i64; 5] = [6, 0, 2, 2, 0];
    let add = REM_TABLE[(n % 5) as usize];
    (start + add - 1) % 10 + 1
}

fn p2_score_at_n_turns(start: i64, n: i64) -> i64 {
    const REM_TABLE: [i64; 20] = [5, 8, 9, 8, 5, 5, 3, 9, 3, 5, 0, 3, 4, 3, 0, 0, 8, 4, 8, 0];
    let add = REM_TABLE[(n % 20) as usize];
    (start + add - 1) % 10 + 1
}

fn p1(p1_start: i64, p2_start: i64) -> i64 {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut turn = 0;
    let mut rolls = 0;
    loop {
        let p1 = p1_score_at_n_turns(p1_start, turn);
        rolls += 1;
        p1_score += p1;
        if p1_score >= 1000 {
            break;
        }
        let p2 = p2_score_at_n_turns(p2_start, turn);
        rolls += 1;
        p2_score += p2;
        if p2_score >= 1000 {
            break;
        }
        turn += 1;
    }

    if p1_score >= 1000 {
        p2_score * rolls * 3
    } else {
        p1_score * rolls * 3
    }
}

const DIE_PERMS: [[u64; 3]; 27] = [
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 3],
    [1, 2, 1],
    [1, 2, 2],
    [1, 2, 3],
    [1, 3, 1],
    [1, 3, 2],
    [1, 3, 3],
    [2, 1, 1],
    [2, 1, 2],
    [2, 1, 3],
    [2, 2, 1],
    [2, 2, 2],
    [2, 2, 3],
    [2, 3, 1],
    [2, 3, 2],
    [2, 3, 3],
    [3, 1, 1],
    [3, 1, 2],
    [3, 1, 3],
    [3, 2, 1],
    [3, 2, 2],
    [3, 2, 3],
    [3, 3, 1],
    [3, 3, 2],
    [3, 3, 3],
];

fn p2(p1_start: u64, p2_start: u64) -> u64 {
    let mut p1_counts = std::collections::HashMap::<(u64, u64, u64), u64>::new();
    p1_counts.insert((0, p1_start, 0), 1);
    let mut p2_counts = std::collections::HashMap::<(u64, u64, u64), u64>::new();
    p2_counts.insert((0, p2_start, 1), 1);
    let mut last_p1 = 0;
    let mut last_p2 = 0;
    loop {
        // termination condition is when we have no new events crossing 21. We do
        // so by evaluating the count maps
        if last_p1 == p1_counts.len() && last_p2 == p2_counts.len() {
            break;
        }
        last_p1 = p1_counts.len();
        last_p2 = p2_counts.len();

        {
            let mut new_counts = std::collections::HashMap::new();
            for ((score, pos, turns), v) in p1_counts.iter() {
                // don't propagate event counts past 21
                if *score >= 21 {
                    continue;
                }
                // Compute all possibilities for this turn, score, and position
                for perm in DIE_PERMS.iter() {
                    let roll = perm.iter().sum::<u64>();
                    let new_pos = (*pos + roll - 1) % 10 + 1;
                    let new_score = *score + new_pos;
                    *new_counts
                        .entry((new_score, new_pos, turns + 2))
                        .or_insert(0) += v;
                }
            }
            // Add new events
            for (k, v) in new_counts {
                if !p1_counts.contains_key(&k) {
                    p1_counts.insert(k, v);
                }
            }
        }

        // Do the same for player 2
        {
            let mut new_counts = std::collections::HashMap::new();
            for ((score, pos, turns), v) in p2_counts.iter() {
                if *score >= 21 {
                    continue;
                }
                for perm in DIE_PERMS.iter() {
                    let roll = perm.iter().sum::<u64>();
                    let new_pos = (*pos + roll - 1) % 10 + 1;
                    let new_score = *score + new_pos;
                    *new_counts
                        .entry((new_score, new_pos, turns + 2))
                        .or_insert(0) += v;
                }
            }
            for (k, v) in new_counts {
                if !p2_counts.contains_key(&k) {
                    p2_counts.insert(k, v);
                }
            }
        }
    }

    // px_counts hold all the different ways (as counts) to get to each possible combination of
    // score, position, and number of turns, so no we must count winning events

    let mut p1_wins = 0;
    for ((p1_score, _, p1_turns), p1_count) in p1_counts.iter() {
        if *p1_score < 21 {
            // If a score is less than 21, it's not a winning event
            continue;
        }
        for ((p2_score, _, p2_turns), p2_count) in p2_counts.iter() {
            if *p1_turns - 1 != *p2_turns || *p2_score >= 21 {
                // - If the other player is already at 21, we already lost.
                // - Only count events where we _just_ won
                continue;
            }
            p1_wins += p1_count * p2_count;
        }
    }

    let mut p2_wins = 0;
    for ((p2_score, _, p2_turns), p2_count) in p2_counts.iter() {
        if *p2_score < 21 {
            continue;
        }
        for ((p1_score, _, p1_turns), p1_count) in p1_counts.iter() {
            if *p2_turns - 1 != *p1_turns || *p1_score >= 21 {
                continue;
            }
            p2_wins += p1_count * p2_count;
        }
    }
    p1_wins.max(p2_wins)
}

fn main() {
    println!("p1: {}", p1(2, 7));
    println!("p2: {}", p2(2, 7));
}

#[cfg(test)]
mod test_day21 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(p1(4, 8), 739785);
        assert_eq!(p2(4, 8), 444356092776315);
    }
}
