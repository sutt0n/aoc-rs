use std::fs::File;
use std::io::prelude::*;

extern crate fancy_regex;

use fancy_regex::Regex;

// #[derive(Debug)]
// struct Elf {
//     snacks: Vec<Snack>,
// }
//
// #[derive(Debug, Clone)]
// struct Snack {
//     calories: u32,
// }
//
// impl Elf {
//     pub fn total_calories(&self) -> u32 {
//         let snacks = self.snacks.clone();
//         let snacks: Vec<u32> = snacks.into_iter().map(|snack| snack.calories).collect();
//         snacks.into_iter().sum()
//     }
// }

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() -> Result<(), std::io::Error> {
    let calibration_values = read_file("input.txt");

    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine|zero))").unwrap();

    let mut calibration_numbers: Vec<i32> = Vec::new();

    for calibration_value in calibration_values.lines() {
        let mut numbers: Vec<i32> = Vec::new();

        for cap in re.captures_iter(calibration_value) {
            let capture = cap.unwrap().get(1).unwrap().as_str();

            let number = match capture {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                // default is number
                n => {
                    let number = n.parse::<i32>().unwrap();
                    number
                }
            };
            numbers.push(number);
        }

        let first_number = numbers[0];
        let last_number = numbers[numbers.len() - 1];

        let number_str = format!("{}{}", first_number.to_string(), last_number.to_string());

        let number = number_str.parse::<i32>().unwrap();

        calibration_numbers.push(number);
    }

    let sum: i32 = calibration_numbers.into_iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}
