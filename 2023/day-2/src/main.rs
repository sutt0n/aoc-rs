use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Colors = (u32, u32, u32);

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_game(game: &str) -> Vec<Colors> {
    game.trim()
        .split(";")
        .map(|set| {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            set.trim().split(",").for_each(|subset| {
                let (count, color) = sscanf::scanf!(subset.trim(), "{u32} {str}").unwrap();
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => panic!("Unknown color"),
                }
            });

            (red, green, blue)
        })
        .collect::<Vec<Colors>>()
}

fn parse_input(input: &str) -> HashMap<usize, Vec<Colors>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let id = sscanf::scanf!(left, "Game {usize}").unwrap();
            let game = parse_game(right);
            (id, game)
        })
        .collect::<HashMap<usize, Vec<Colors>>>()
}

fn part1(input: &str) {
    let games = parse_input(input);

    let mut impossible_ids = Vec::new();

    'outer: for (id, game) in games.iter() {
        for (red, green, blue) in game.iter() {
            if *red > 12 || *green > 13 || *blue > 14 {
                impossible_ids.push(*id);
                continue 'outer;
            }
        }
    }

    // sum game
    let sum = games.iter().map(|(id, _)| *id).sum::<usize>();

    println!("Sum: {}", sum - impossible_ids.iter().sum::<usize>());
}

fn part2(input: &str) {
    let games = parse_input(input);

    // sum the powers
    let powers = games
        .iter()
        .map(|(_, game)| {
            let max_red = game.iter().map(|(red, _, _)| red).max().unwrap();
            let max_green = game.iter().map(|(_, green, _)| green).max().unwrap();
            let max_blue = game.iter().map(|(_, _, blue)| blue).max().unwrap();
            
            max_red * max_green * max_blue
        })
        .sum::<u32>();

    println!("Powers: {}", powers);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
