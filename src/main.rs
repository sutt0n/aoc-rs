use std::fs::File;
use std::io::prelude::*;

trait Player<T> {
    fn play(&self, opponent: T) -> u32;
    fn get_action_score(&self, action: Action) -> u32;
    fn get_outcome_score(&self, outcome: Outcome) -> u32;
    fn get_player_outcome(&self, opponent_action: Action) -> Outcome;
    fn get_player_action_for_outcome(&self, opponent_action: Action, outcome: Outcome) -> Action;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    None,
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
pub struct Elf {
    action: Action,
    score: u32,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Elf {
    pub fn new(action: Action) -> Elf {
        Elf { action, score: 0 }
    }
}

impl Player<Elf> for Elf {
    fn play(&self, opponent: Elf) -> u32 {
        // get action score for both players
        // get outcome for the actions
        // get outcome score
        // return sum
        let opponent_action = opponent.action;

        let player_action_score = self.get_action_score(self.action);
        // let opponent_action_score = opponent.get_action_score(opponent_action);
        let get_player_outcome = self.get_player_outcome(opponent_action);
        let player_outcome_score = self.get_outcome_score(get_player_outcome);

        // Elf {
        player_action_score + player_outcome_score
        // action: self.action,
        // }
    }

    fn get_action_score(&self, action: Action) -> u32 {
        match action {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
            Action::None => 0,
        }
    }

    fn get_outcome_score(&self, outcome: Outcome) -> u32 {
        match outcome {
            Outcome::Draw => 3,
            Outcome::Loss => 0,
            Outcome::Win => 6,
        }
    }

    fn get_player_action_for_outcome(&self, opponent_action: Action, outcome: Outcome) -> Action {
        match outcome {
            Outcome::Draw => match opponent_action {
                Action::Rock => Action::Rock,
                Action::Paper => Action::Paper,
                Action::Scissors => Action::Scissors,
                Action::None => panic!("mayne wtf"),
            },
            Outcome::Loss => match opponent_action {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper,
                Action::None => panic!("mayne wtf"),
            },
            Outcome::Win => match opponent_action {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock,
                Action::None => panic!("mayne wtf"),
            },
        }
    }

    fn get_player_outcome(&self, opponent_action: Action) -> Outcome {
        match self.action {
            Action::Rock => match opponent_action {
                Action::Rock => Outcome::Draw,
                Action::Paper => Outcome::Loss,
                Action::Scissors => Outcome::Win,
                Action::None => panic!("mayne wtf"),
            },
            Action::Paper => match opponent_action {
                Action::Rock => Outcome::Win,
                Action::Paper => Outcome::Draw,
                Action::Scissors => Outcome::Loss,
                Action::None => panic!("mayne wtf"),
            },
            Action::Scissors => match opponent_action {
                Action::Rock => Outcome::Loss,
                Action::Paper => Outcome::Win,
                Action::Scissors => Outcome::Draw,
                Action::None => panic!("mayne wtf"),
            },
            Action::None => panic!("mayne wtf"),
        }
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_strategy_guide(contents: &str) -> u32 {
    let mut player = Elf::new(Action::None);
    let mut opponent = Elf::new(Action::None);
    let mut total_score = 0;

    for line in contents.lines() {
        let (str_opp_action, str_player_action) = line.split_at(2);

        let opponent_action = match str_opp_action.trim() {
            "A" => Action::Rock,
            "B" => Action::Paper,
            "C" => Action::Scissors,
            &_ => todo!(),
        };
        // Part 1
        // let player_action = match str_player_action.trim() {
        //     "Y" => Action::Paper,
        //     "X" => Action::Rock,
        //     "Z" => Action::Scissors,
        //     &_ => todo!(),
        // };

        // Part 2
        let player_outcome = match str_player_action.trim() {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            &_ => todo!(),
        };

        let player_action = player.get_player_action_for_outcome(opponent_action, player_outcome);

        player.action = player_action;
        opponent.action = opponent_action;

        total_score += player.play(opponent);
    }

    total_score
}

fn main() {
    let contents = read_file("input.txt");
    let score = parse_strategy_guide(contents.as_str());

    println!("score is {}", score)
}
