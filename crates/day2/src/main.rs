use std::str::FromStr;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

/// https://adventofcode.com/2024/day/2
fn main() {
    assert_eq!(part_one(EXAMPLE), 2);
    println!("{}", part_one(INPUT));
    assert_eq!(part_one(INPUT), 279);

    assert_eq!(part_two(EXAMPLE), 4);
    println!("{}", part_two(INPUT));
    assert_eq!(part_two(EXAMPLE), 343);
}

fn part_one(input: &str) -> i32 {
    get_levels(input)
        .into_iter()
        .filter(|levels| is_safe(&parse_diffs(levels)))
        .count() as i32
}

fn part_two(input: &str) -> i32 {
    get_levels(input)
        .into_iter()
        .filter(|levels| is_safe_dampened(levels))
        .count() as i32
}

fn get_levels(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|list| {
            list.into_iter()
                .map(|s| i32::from_str(s).unwrap())
                .collect()
        })
        .collect()
}

fn parse_diffs(levels: &Vec<i32>) -> Vec<i32> {
    levels
        .windows(2)
        .map(|chunk| chunk[0] - chunk[1])
        .collect::<Vec<_>>()
}

/// Part One
fn is_safe(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|&it| it >= 1 && it <= 3) || diffs.iter().all(|&it| it <= -1 && it >= -3)
}

/// Part Two
fn is_safe_dampened(levels: &Vec<i32>) -> bool {
    if is_safe(&parse_diffs(levels)) {
        return true;
    }

    // Brute Force :)
    levels
        .into_iter()
        .enumerate()
        .map(|(index, level)| {
            let mut temp = levels.clone();
            temp.remove(index);
            temp
        })
        .any(|new_levels| is_safe(&parse_diffs(&new_levels)))
}
