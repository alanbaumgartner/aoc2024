use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

fn main() {
    assert_eq!(part_one(EXAMPLE), 11);
    println!("{}", part_one(INPUT));

    assert_eq!(part_two(EXAMPLE), 31);
    println!("{}", part_two(INPUT));
}

fn part_one(input: &str) -> i32 {
    let (first, second) = get_lists(input);

    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(i, j)| (i - j).abs())
        .sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    let (first, second) = get_lists(input);

    let counts = second.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    first
        .into_iter()
        .map(|location| location * counts.get(&location).unwrap_or(&0))
        .sum::<i32>()
}

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = vec![];
    let mut second = vec![];

    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .for_each(|vec| {
            first.push(vec[0].parse::<i32>().unwrap());
            second.push(vec[1].parse::<i32>().unwrap());
        });

    first.sort();
    second.sort();

    (first, second)
}
