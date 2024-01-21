use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {
    let nice_strings = input
        .lines()
        .into_iter()
        .filter(|line| {
            let vowel_count = line
                .chars()
                .filter(|c| {
                    let vowels = ['a', 'e', 'i', 'o', 'u'];
                    vowels.contains(c)
                })
                .count();

            let has_double_letters = line
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .filter(|ll| ll[0] == ll[1])
                .count();

            let forbidden = ["ab", "cd", "pq", "xy"];
            let has_forbidden = forbidden
                .iter()
                .filter(|ll| line.contains(*ll))
                .count();


            vowel_count >= 3 && has_double_letters >= 1 && has_forbidden == 0
        })
        .count();

    println!("nice strings {}", nice_strings);
}

fn part2(input: &str) {
    let nice_strings = input
        .lines()
        .into_iter()
        .filter(|line| {
            let mut pairs = HashMap::new();
            let mut has_pair = false;
            let mut has_repeat = false;

            for (i, c) in line.chars().enumerate() {
                if i > 0 {
                    let pair = format!("{}{}", line.chars().nth(i - 1).unwrap(), c);
                    if pairs.contains_key(&pair) {
                        let index = pairs.get(&pair).unwrap();
                        if i - index > 1 {
                            has_pair = true;
                        }
                    } else {
                        pairs.insert(pair, i);
                    }
                }

                if i > 1 {
                    if line.chars().nth(i - 2).unwrap() == c {
                        has_repeat = true;
                    }
                }
            }

            has_pair && has_repeat
        })
        .count();

    println!("nice strings {}", nice_strings);
}


fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
