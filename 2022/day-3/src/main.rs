#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::prelude::*;

fn parse_knapsacks(contents: String) -> Vec<char> {
    let mut ret = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

        for letter in first_compartment.chars() {
            if second_compartment.contains(letter) {
                ret.push(letter);
                break;
            }
        }
    }

    ret
}

fn find_duplicate(chars: Vec<&str>) -> char {
    let other_sets: Vec<Vec<char>> = chars
        .iter()
        .skip(1)
        .map(|&s| s.chars().collect::<Vec<char>>())
        .collect();

    // check if first vec of chars contains any chars in other_sets
    chars[0]
        .chars()
        .find(|c| other_sets.iter().all(|other_set| other_set.contains(c)))
        .unwrap()
}

fn parse_knapsacks_badge(contents: String) -> Vec<char> {
    let mut ret = Vec::new();

    for lines in contents.lines().array_chunks::<3>() {
        let line: Vec<&str> = lines.into();
        let duplicate = find_duplicate(lines.into());

        ret.push(duplicate);
    }

    ret
}

fn get_priority(item: char) -> u16 {
    match item.is_uppercase() {
        true => 27 + (item as u16) - ('A' as u16),
        false => 1 + (item as u16) - ('a' as u16),
    }
}

fn sum_knapsack_items(items: Vec<char>) -> u32 {
    let mut sum = 0;

    for item in items {
        let priority = get_priority(item);

        sum += <u16 as Into<u32>>::into(priority);
    }

    sum
}

fn main() {
    let contents = read_file("input.txt");
    let badge_letters = parse_knapsacks_badge(contents.clone());
    let individual_knapsack_letters = parse_knapsacks(contents.clone());
    let sum_knapsacks = sum_knapsack_items(individual_knapsack_letters.clone());
    let sum_badges = sum_knapsack_items(badge_letters);
    println!("sum_knapsacks {}", sum_knapsacks);
    // let score = parse_strategy_guide(contents.as_str());
    println!("sum_badges {}", sum_badges);
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_sum_knapsack_items() {
        let letters = parse_knapsacks(String::from(EXAMPLE));
        let sum_items = sum_knapsack_items(letters.clone());
        assert_eq!(sum_items, 157);
    }

    #[test]
    fn test_sum_knapsack_badges() {
        let letters = parse_knapsacks_badge(String::from(EXAMPLE));
        println!("letters {:?}", letters);
        let sum_items = sum_knapsack_items(letters.clone());
        assert_eq!(sum_items, 70);
    }
}
