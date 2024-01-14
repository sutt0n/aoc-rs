use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);
type Part = Vec<(Pos, u32)>;

fn concat(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, &x| acc * 10 + x)
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_possible_parts(input: &str) -> Vec<Part> {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Grid>();
    let mut possible_parts = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut part: Part = Vec::new();
        for (x, character) in row.iter().enumerate() {
            if character.is_digit(10) {
                let num = character.to_digit(10).unwrap();
                part.push(((x, y), num));
            } else if !part.is_empty() {
                possible_parts.push(part.clone());
                part.clear();
            }
        }
        if !part.is_empty() {
            possible_parts.push(part.clone());
        }
    }

    possible_parts
}

fn get_valid_neighbors(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let max_y = grid.len();
    let max_x = grid[0].len();

    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(x, y)| (pos.0 as i32 + x, pos.1 as i32 + y))
        .filter_map(|(x, y)| {
            if x < 0 || y < 0 || x >= max_x as i32 || y >= max_y as i32 {
                None
            } else {
                Some((x as usize, y as usize))
            }
        })
        .collect_vec()
}

fn is_part_adjacent_to_symbol(part: &Part, grid: &Grid) -> bool {
    part.iter()
        .flat_map(|((x, y), _)| {
            get_valid_neighbors((*x, *y), grid).into_iter().map(|(mx, my)| {
                if grid[my][mx].is_digit(10) || grid[my][mx] == '.' {
                    return false;
                } else {
                    return true;
                }
            })
        })
        .any(|x| x)
}

fn part1(input: &str) {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Grid>();

    let nums = get_possible_parts(input)
        .iter()
        .filter(|&part| is_part_adjacent_to_symbol(part, &grid))
        .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
        .map(|vec| concat(&vec))
        .collect_vec();

    println!("{:?}", nums.iter().sum::<u32>());
}

fn part2(input: &str) {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Grid>();
    let possible_parts = get_possible_parts(input);
    let possible_gears = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, character)| {
                if character == &'*' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    let mut ratios: Vec<u32> = Vec::new();

    for gear_pos in possible_gears {
        let mut gear_nums: HashSet<Part> = HashSet::new();

        for (x, y) in get_valid_neighbors(gear_pos, &grid) {
            if grid[y][x].is_digit(10) {
                for part_nums in &possible_parts {
                    if part_nums.iter().any(|((px, py), _)| *px == x && *py == y) {
                        gear_nums.insert(part_nums.clone());
                    }
                }
            }
        }

        if gear_nums.len() == 2 {
            let ratio = gear_nums
                .iter()
                .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
                .map(|vec| concat(&vec))
                .product();

            ratios.push(ratio);
        }
    }

    println!("{:?}", ratios.iter().sum::<u32>());
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
