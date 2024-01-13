use std::fs::File;
use std::io::prelude::*;

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
