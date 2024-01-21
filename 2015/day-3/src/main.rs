use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {
    let houses = input
        .chars()
        .map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        })
        .scan((0, 0), |state, (x, y)| {
            state.0 += x;
            state.1 += y;
            Some(*state)
        })
        .collect::<Vec<_>>();

    let mut visited = std::collections::HashSet::new();

    for house in houses {
        visited.insert(house);
    }

    println!("sum {}", visited.len());
}

fn visit_houses<F>(input: &str, f: F) -> Vec<(i32, i32)> where
    F: Fn(usize, char) -> Option<(i32, i32)> {
    input
        .chars()
        .enumerate()
        .map(|(idx, c)| f(idx, c))
        .flat_map(|x| x)
        .scan((0, 0), |state, (x, y)| {
            state.0 += x;
            state.1 += y;
            Some(*state)
        })
        .collect::<Vec<(i32, i32)>>()
}

fn part2(input: &str) {
    let santa_houses = visit_houses(input, |idx, c| {
        if idx % 2 == 0 {
            return Some(match c {
                '^' => (0, 1),
                'v' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 0),
            });
        }
        None
    });

    let robo_houses = visit_houses(input, |idx, c| {
        if idx % 2 != 0 {
            return Some(match c {
                '^' => (0, 1),
                'v' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 0),
            });
        }
        None
    });

    let mut visited = std::collections::HashSet::new();

    for house in santa_houses {
        visited.insert(house);
    }

    for house in robo_houses {
        visited.insert(house);
    }

    println!("sum {}", visited.len());
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
