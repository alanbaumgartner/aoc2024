use regex::Regex;

const EXAMPLE: &str = include_str!("example");
const EXAMPLE2: &str = include_str!("example2");
const INPUT: &str = include_str!("input");

/// https://adventofcode.com/2024/day/3
fn main() {
    assert_eq!(part_one(EXAMPLE), 161);
    println!("{}", part_one(INPUT));
    assert_eq!(part_one(INPUT), 173517243);

    assert_eq!(part_two(EXAMPLE2), 48);
    println!("{}", part_two(INPUT));
    assert_eq!(part_two(INPUT), 100450138);
}

fn part_one(input: &str) -> i32 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    regex
        .captures_iter(input)
        .into_iter()
        .map(|c| {
            let first = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let second = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            first * second
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;

    for capture in regex.captures_iter(input) {
        let whole = capture.get(0).unwrap().as_str();

        match whole {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if mul_enabled {
                    let first = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let second = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    sum += first * second
                }
            }
        }
    }

    sum
}
