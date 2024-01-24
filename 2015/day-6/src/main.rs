use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {
    let mut rows = vec![false; 1000];
    let mut lights = vec![rows; 1000];

    let regex = regex::Regex::new(r"turn on|turn off|toggle").unwrap();

    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let mode = caps.get(0).unwrap().as_str();

        let (_, x1, y1, x2, y2) = sscanf::scanf!(line, "{str} {usize},{usize} through {usize},{usize}").unwrap();

        match mode {
            "turn off" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        lights[x][y] = false;
                    }
                }
            },
            "turn on" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        lights[x][y] = true;
                    }
                }
            },
            "toggle" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        let v = lights[x][y];
                        lights[x][y] = !v;
                    }
                }
            },
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    let mut sum = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] == true {
                sum += 1;
            }
        }
    }

    println!("sum {}", sum);
}

fn part2(input: &str) {
    let mut rows = vec![0; 1000];
    let mut lights = vec![rows; 1000];

    let regex = regex::Regex::new(r"turn on|turn off|toggle").unwrap();

    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let mode = caps.get(0).unwrap().as_str();

        let (_, x1, y1, x2, y2) = sscanf::scanf!(line, "{str} {usize},{usize} through {usize},{usize}").unwrap();

        match mode {
            "turn off" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        let v = lights[x][y] - 1;
                        if v < 0 {
                            lights[x][y] = 0;
                        } else {
                            lights[x][y] = v;
                        }
                    }
                }
            },
            "turn on" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        lights[x][y] += 1;
                    }
                }
            },
            "toggle" => {
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        lights[x][y] += 2;
                    }
                }
            },
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    let mut sum: i64 = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            sum += lights[x][y];
        }
    }

    println!("sum {}", sum);
}


fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
