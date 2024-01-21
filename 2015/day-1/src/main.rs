use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {
    let mut floor = 0;

    let sum = input
        .chars()
        .map(|c| {
            match c {
                ')' => -1,
                '(' => 1,
                _ => 0,
            }
        })
        .sum::<i32>();

    println!("sum {}", sum);
}

fn part2(input: &str) {
    let mut floor = 0;
    let mut position = 0;

    for (idx, char) in input.chars().enumerate() {
        if floor == -1 {
            position = idx;
            break;
        }

        floor += match char {
            ')' => -1,
            '(' => 1,
            _ => 0,
        };

    }


    println!("position {} {}", position, floor);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
