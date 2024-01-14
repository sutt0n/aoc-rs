use std::cmp::min;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

// fn concat(vec: &[u32]) -> u32 {
//     vec.iter().fold(0, |acc, &x| acc * 10 + x)
// }

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// (points, card_id)
fn parse_card_points(winning_numbers: Vec<u32>, play_numbers: Vec<u32>) -> u32 {
    let winning_numbers_played: Vec<u32> = winning_numbers
        .iter()
        .filter_map(|winning_number| {
            play_numbers
                .clone()
                .into_iter()
                .find(|play_number| {
                    winning_number == play_number
                })
        })
        .collect::<Vec<u32>>();

    winning_numbers_played.len() as u32
}

fn get_card_points(input: &str) -> Vec<u32> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (play_numbers, winning_numbers) = numbers.trim().split_once('|').unwrap();

            let play_numbers: Vec<u32> = play_numbers
                .trim()
                .split(' ')
                .filter(|num| {
                    num.trim() != ""
                })
                .into_iter()
                .map(|num| {
                    num.trim().parse::<u32>().unwrap()
                })
                .collect();

            let winning_numbers = winning_numbers
                .trim()
                .split(' ')
                .filter(|num| {
                    num.trim() != ""
                })
                .into_iter()
                .map(|num| {
                    num.trim().parse::<u32>().unwrap()
                })
                .collect();

            parse_card_points(winning_numbers, play_numbers)
        })
        .collect()
}

fn part1(input: &str) {
    let card_points = get_card_points(input);
    let points_sum = card_points
        .iter()
        .map(|points| {
            if *points == 0 {
                return 0;
            }
            u32::pow(2, *points - 1)
        })
        .sum::<u32>();

    println!("Sum: {}", points_sum);
}

fn part2(input: &str) {
    let card_points = get_card_points(input);

    let winnings_numbers: Vec<Vec<u32>> = card_points
        .iter()
        .enumerate()
        .map(|(idx, points)| {
            let mut v = Vec::new();
            for x in 1..=*points {
                v.push(idx as u32 + x + 1);
            }
            v
        })
        .collect();

    let mut card_values: HashMap<u32, u32> = HashMap::new();

    for (idx, _) in card_points.iter().enumerate() {
        let card_id = idx as u32 + 1;
        *card_values.entry(card_id).or_default() += 1;
    }

    for (idx, winnings) in winnings_numbers.iter().enumerate() {
        let card_id = idx as u32 + 1;
        for card in winnings {
            *card_values.entry(*card).or_default() += *card_values.get(&(&card_id)).unwrap();
        }
    }

    let sum: u32 = card_values
        .values()
        .into_iter()
        .sum();

    println!("sum {:?}", sum);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
