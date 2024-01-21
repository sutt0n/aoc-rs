use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn part1(input: &str) {

}

fn part2(input: &str) {
}


fn main() -> Result<(), std::io::Error> {
    let input = read_file("input.txt");

    part1(&input);
    part2(&input);

    Ok(())
}
