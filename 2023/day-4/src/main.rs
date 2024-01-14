use std::fs::File;
use std::io::prelude::*;

fn concat(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, &x| acc * 10 + x)
}

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

fn part1(input: &str) {
    let card_points: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let (card_id, numbers) = line.split_once(":").unwrap();
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
        .collect();

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
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    // part2(&input);

    Ok(())
}
