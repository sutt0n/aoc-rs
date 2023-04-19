use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Elf {
    snacks: Vec<Snack>,
}

#[derive(Debug, Clone)]
struct Snack {
    calories: u32,
}

impl Elf {
    pub fn total_calories(&self) -> u32 {
        let snacks = self.snacks.clone();
        let snacks: Vec<u32> = snacks.into_iter().map(|snack| snack.calories).collect();
        snacks.into_iter().sum()
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_elves_and_snacks(contents: &str) -> Vec<Elf> {
    let mut snacks: Vec<Snack> = Vec::new();
    let mut elves = Vec::new();
    let mut elf_needs_push = true;

    for line in contents.lines() {
        if line.len() > 0 {
            snacks.push(Snack {
                calories: line.parse::<u32>().unwrap(),
            });
            elf_needs_push = true;
        } else {
            elves.push(Elf {
                snacks: snacks.clone(),
            });
            snacks.clear();
            elf_needs_push = false;
        }
    }

    if elf_needs_push {
        elves.push(Elf { snacks });
    }

    elves
}

fn get_top_3_elves_calories_sum(elves_calories: Vec<u32>) -> [u32; 3] {
    // horrible, but it works
    let mut elves_calories: Vec<u32> = elves_calories.clone();
    elves_calories.sort_unstable_by(|a, b| a.cmp(b));
    elves_calories.reverse();

    [
        elves_calories.remove(0),
        elves_calories.remove(0),
        elves_calories.remove(0),
    ]
}

fn main() {
    let contents = read_file("input.txt");
    let elves = parse_elves_and_snacks(contents.as_str());

    let elves_calories: Vec<u32> = elves.into_iter().map(|x| x.total_calories()).collect();
    let most_calories = elves_calories.iter().max().unwrap();
    let elf_with_most_calories = elves_calories
        .iter()
        .position(|&c| c == *most_calories)
        .unwrap();

    println!(
        "elf with most calories is {} with {}",
        elf_with_most_calories, most_calories
    );

    let top_3_elves = get_top_3_elves_calories_sum(elves_calories)
        .into_iter()
        .sum::<u32>();

    println!("top 3 total is {}", top_3_elves)
}
