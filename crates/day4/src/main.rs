const EXAMPLE: &str = include_str!("example");
const INPUT: &str = include_str!("input");

/// https://adventofcode.com/2024/day/4
/// We ain't happy with this one, but whats done is done...
fn main() {
    assert_eq!(part_one(EXAMPLE), 18);
    println!("{}", part_one(INPUT));
    assert_eq!(part_one(INPUT), 2454);

    assert_eq!(part_two(EXAMPLE), 9);
    println!("{}", part_two(INPUT));
    assert_eq!(part_two(INPUT), 1858);
}

fn part_one(input: &str) -> i32 {
    let grid = parse_input(input);
    let starting_positions = part1::x_positions(&grid);
    part1::find_xmas_count(grid, starting_positions)
}

fn part_two(input: &str) -> i32 {
    let grid = parse_input(input);
    let starting_positions = part2::a_positions(&grid);
    part2::find_xmas_count(grid, starting_positions)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn char_at(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Option<char> {
    grid.get(x).and_then(|row| row.get(y)).cloned()
}

mod part1 {
    use crate::char_at;

    pub(super) fn x_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        grid.into_iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.into_iter()
                    .enumerate()
                    .flat_map(|(y, col)| if col.eq(&'X') { Some((x, y)) } else { None })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }

    pub(super) fn find_xmas_count(
        grid: Vec<Vec<char>>,
        starting_positions: Vec<(usize, usize)>,
    ) -> i32 {
        let mut count = 0;

        let dimensions = (grid.len(), grid[0].len());

        for starting_position in starting_positions {
            let possible_positions = possible_positions(starting_position, dimensions);
            for possible_position in possible_positions {
                if matches_positions(&grid, possible_position) {
                    count += 1;
                }
            }
        }

        count
    }

    fn possible_positions(
        (x, y): (usize, usize),
        (max_x, max_y): (usize, usize),
    ) -> Vec<Vec<(usize, usize)>> {
        let mut possible_positions = Vec::new();

        if x >= 3 {
            possible_positions.push(vec![(x - 1, y), (x - 2, y), (x - 3, y)]);

            if y >= 3 {
                possible_positions.push(vec![(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]);
            }

            if y + 3 <= max_y {
                possible_positions.push(vec![(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]);
            }
        }

        if y >= 3 {
            possible_positions.push(vec![(x, y - 1), (x, y - 2), (x, y - 3)]);
        }

        if x + 3 <= max_x {
            possible_positions.push(vec![(x + 1, y), (x + 2, y), (x + 3, y)]);

            if y >= 3 {
                possible_positions.push(vec![(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]);
            }

            if y + 3 <= max_y {
                possible_positions.push(vec![(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]);
            }
        }

        if y + 3 <= max_y {
            possible_positions.push(vec![(x, y + 1), (x, y + 2), (x, y + 3)]);
        }

        possible_positions
    }

    fn matches_positions(grid: &Vec<Vec<char>>, positions: Vec<(usize, usize)>) -> bool {
        char_at(grid, positions[0]) == Some('M')
            && char_at(grid, positions[1]) == Some('A')
            && char_at(grid, positions[2]) == Some('S')
    }
}

mod part2 {
    use crate::char_at;

    pub(super) fn a_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        grid.into_iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.into_iter()
                    .enumerate()
                    .flat_map(|(y, col)| if col.eq(&'A') { Some((x, y)) } else { None })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }

    pub(super) fn find_xmas_count(
        grid: Vec<Vec<char>>,
        starting_positions: Vec<(usize, usize)>,
    ) -> i32 {
        let mut count = 0;

        let dimensions = (grid.len(), grid[0].len());

        for starting_position in starting_positions {
            let possible_positions = possible_positions(starting_position, dimensions);
            for possible_position in possible_positions.into_iter().flatten() {
                if matches_positions(&grid, possible_position) {
                    count += 1;
                }
            }
        }

        count
    }

    fn possible_positions(
        (x, y): (usize, usize),
        (max_x, max_y): (usize, usize),
    ) -> Vec<Vec<Vec<(usize, usize)>>> {
        let mut possible_positions = Vec::new();

        if x >= 1 && y >= 1 && x + 1 <= max_x && y + 1 <= max_y {
            possible_positions.push(permutations((x, y)));
        }

        possible_positions
    }

    fn permutations((x, y): (usize, usize)) -> Vec<Vec<(usize, usize)>> {
        let mut permutations = Vec::new();

        let north_west = (x - 1, y + 1);
        let north_east = (x + 1, y + 1);
        let south_west = (x - 1, y - 1);
        let south_east = (x + 1, y - 1);

        // M # S
        // # A #
        // M # S

        permutations.push(vec![
            north_west, south_east, south_west, north_east,
        ]);

        // M # M
        // # A #
        // S # S
        permutations.push(vec![
            north_west, south_east,north_east, south_west,
        ]);

        // S # M
        // # A #
        // S # M
        permutations.push(vec![
            north_east, south_west, south_east, north_west,
        ]);

        // S # S
        // # A #
        // M # M
        permutations.push(vec![
            south_east, north_west, south_west, north_east,
        ]);

        permutations
    }

    fn matches_positions(grid: &Vec<Vec<char>>, positions: Vec<(usize, usize)>) -> bool {
        char_at(grid, positions[0]) == Some('M')
            && char_at(grid, positions[1]) == Some('S')
            && char_at(grid, positions[2]) == Some('M')
            && char_at(grid, positions[3]) == Some('S')
    }
}
