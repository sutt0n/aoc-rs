use std::fs::File;
use std::io::prelude::*;


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

#[derive(Debug, Clone, Copy)]
struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32,
}

impl Game {
    pub fn new(id: i32, blue: i32, red: i32, green: i32) -> Game {
        Game {
            id,
            blue,
            red,
            green,
        }
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn generate_combinations(objects: &Vec<Game>, current: Vec<Game>, index: usize, target_red: i32, target_green: i32, target_blue: i32) -> Vec<Vec<Game>> {
    let mut combinations: Vec<Vec<Game>> = Vec::new();

    if current.len() == 3 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for game in current.clone().into_iter() {
            red += game.red;
            green += game.green;
            blue += game.blue;
        }

        if red == target_red && green == target_green && blue == target_blue {
            combinations.push(current);
        }
    } else {
        for i in index..objects.len() {
            let mut current = current.clone();
            current.push(objects[i].clone());
            combinations.append(&mut generate_combinations(objects, current, i + 1, target_red, target_green, target_blue));
        }
    }

    combinations
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("test-input.txt");

    let mut games: Vec<Game> = Vec::new();

    let target_red = 12;
    let target_blue = 13;
    let target_green = 14;

    for lines in input.lines() {
        let mut game = lines.split(":");
        let id = game.next().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        let subsets = game.next().unwrap().split(";");

        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for subset in subsets {
            let mut colors = subset.split(",");
            println!("{:?}", colors);
            let count = colors.next().unwrap().trim().parse::<i32>().unwrap();
            let color = colors.next().unwrap().trim();

            match color {
                "blue," => blue += count,
                "red," => red += count,
                "green," => green += count,
                _ => (),
            }
        }

        let game = Game { id, blue, red, green };
        games.push(game);
    }

    let mut game_ids: Vec<i32> = Vec::new();

    for game in games.clone().into_iter() {
        if game.blue < target_blue && game.red < target_red && game.green < target_green {
            game_ids.push(game.id); 
        }
    }

    // sum of ids
    let sum: i32 = game_ids.into_iter().sum();
    println!("Sum of ids: {}", sum);

    Ok(())
}
