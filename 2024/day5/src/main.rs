use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

/// https://adventofcode.com/2024/day/5
fn main() {
    assert_eq!(part1::solve(EXAMPLE), 143);
    println!("{}", part1::solve(INPUT));
    assert_eq!(part1::solve(INPUT), 5391);

    assert_eq!(part2::solve(EXAMPLE), 123);
    println!("{}", part2::solve(INPUT));
    assert_eq!(part2::solve(INPUT), 1858);
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let lines = input.lines();

    let mut pairs = HashMap::new();

    for line in lines.clone().take_while(|line| !line.is_empty()) {
        let mut split = line.split("|");
        let first = split.next().unwrap().parse::<i32>().unwrap();
        let second = split.next().unwrap().parse::<i32>().unwrap();

        pairs.entry(first).or_insert_with(Vec::new).push(second);
    }

    let updates = lines
        .skip_while(|it| !it.contains(","))
        .map(|it| {
            it.split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<_>();

    (pairs, updates)
}

mod part1 {
    use crate::parse_input;

    pub(super) fn solve(input: &str) -> i32 {
        let (pairs, updates) = parse_input(input);

        let mut sum = 0;

        for update in updates {
            let mut correct = true;

            for (idx, &page) in update.iter().enumerate() {
                if let Some(before) = pairs.get(&page) {
                    for &b in before {
                        if update
                            .iter()
                            .enumerate()
                            .find(|(_, &it)| it == b)
                            .is_some_and(|(before_idx, &_)| idx > before_idx)
                        {
                            correct = false;
                        }
                    }
                }
            }

            if correct {
                sum += update[update.len() / 2];
            }
        }

        sum
    }
}

mod part2 {
    use crate::parse_input;

    pub(super) fn solve(input: &str) -> i32 {
        let (pairs, updates) = parse_input(input);

        let mut incorrect = vec![];

        for update in updates {
            let mut correct = true;

            for (idx, &page) in update.iter().enumerate() {
                if let Some(before) = pairs.get(&page) {
                    for &b in before {
                        if update
                            .iter()
                            .enumerate()
                            .find(|(_, &it)| it == b)
                            .is_some_and(|(before_idx, &_)| idx > before_idx)
                        {
                            correct = false;
                        }
                    }
                }
            }

            if !correct {
                incorrect.push(update);
            }
        }

        let mut sum = 0;

        for bad in incorrect {
            let mut corrected = bad.clone();

            for (idx, &page) in bad.iter().enumerate() {
                if let Some(before) = pairs.get(&page) {
                    if let Some(min_idx) = before
                        .into_iter()
                        .map(|it| corrected.iter().position(|r| r == it))
                        .flatten()
                        .min()
                    {
                        if idx < min_idx {
                            continue;
                        }

                        corrected.remove(idx);
                        corrected.insert(min_idx, page);
                    }
                }
            }

            sum += corrected[corrected.len() / 2];
        }

        sum
    }
}
