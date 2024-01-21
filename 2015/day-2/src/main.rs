use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|dims| {
            let (l, w, h) = sscanf::sscanf!(dims, "{u32}x{u32}x{u32}").unwrap();
            let min = vec![l * w, w * h, h * l].iter().min().unwrap().clone();
            2*l*w + 2*w*h + 2*h*l + min
        })
        .sum();

    println!("paper {}", sum);
}

fn part2(input: &str) {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|dims| {
            let (l, w, h) = sscanf::sscanf!(dims, "{u32}x{u32}x{u32}").unwrap();
            let mut vec = vec![l,w,h];
            vec.sort();
            let (first, second) = (vec.get(0).unwrap(), vec.get(1).unwrap());
            (l * w * h) + ((first * 2) + (second * 2))
        })
        .sum();

    println!("ribbon {}", sum);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
