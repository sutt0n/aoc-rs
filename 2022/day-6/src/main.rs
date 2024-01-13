#![feature(iter_array_chunks)]

use std::{fs, collections::HashSet};

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn parse(contents: String) -> u32 {
    let s = contents.as_bytes();
    let s = s.windows(14).position(|w| w.iter().collect::<HashSet<_>>().len() == 14).map(|x| x + 14).unwrap();


    println!("result is {}", s);

    s as u32
}

fn main() {
    // let test = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
    let test = read_file("input.txt");
    let result = parse(test);
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1 : &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE2 : &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE3 : &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE4 : &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_parse() {
        let example_1_result = parse(String::from(EXAMPLE1));
        assert_eq!(example_1_result, 5);

        let example_2_result = parse(String::from(EXAMPLE2));
        assert_eq!(example_2_result, 6);

        let example_3_result = parse(String::from(EXAMPLE3));
        assert_eq!(example_3_result, 10);

        let example_4_result = parse(String::from(EXAMPLE4));
        assert_eq!(example_4_result, 11);
    }
}
