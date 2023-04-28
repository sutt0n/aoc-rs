#![feature(iter_array_chunks)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{fs, ops::Deref, thread::current, collections::VecDeque};

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn get_line_length(current_stack: &&str) -> usize {
    current_stack.lines().map(|l| l).collect::<Vec<&str>>().last().unwrap().len()
}

fn get_num_crates(current_stack: &&str) -> usize {
    current_stack.lines().map(|l| l).collect::<Vec<&str>>().last().unwrap().split("   ").collect::<Vec<&str>>().len()
}

fn get_crates_state(current_stack: &&str, num_crates: &usize, line_length: &usize) -> Vec<Vec<char>> {
    let mut crates = vec![vec![]; *num_crates];

    for line in current_stack.lines() {
        for i in 0..*line_length {
            let byte = line.as_bytes()[i] as char;
            let is_letter = byte.is_ascii_uppercase();

            if is_letter {
                let n = i as f32;
                let crate_idx = n * 0.25;
                let crate_idx = crate_idx.floor();

                crates[crate_idx as usize].splice(0..0, vec![byte]);
            }
        }
    }

    crates
}

fn parse_instruction_message(words: &Vec<&str>) -> (i32, i32, i32) {
    let (crate_count, from_stack, to_stack) = (
        words.get(1).unwrap(),
        words.get(3).unwrap(),
        words.get(5).unwrap(),
    );

    let crate_count = crate_count.parse::<i32>().expect("");
    let from_stack = from_stack.parse::<i32>().expect("");
    let to_stack = to_stack.parse::<i32>().expect("");

    (
        crate_count,
        from_stack,
        to_stack,
    )
}

fn exec_crate_instructions_p1<'a>(instructions: &&str, crates: &'a mut [Vec<char>]) -> &'a mut [Vec<char>] {

    for line in instructions.lines() {
        let split_line = line.split(' ');
        let words = split_line.collect::<Vec<&str>>();

        let (number_of_crates, from_stack, to_stack) = parse_instruction_message(&words);
        
        let from_stack = from_stack - 1;
        let to_stack = to_stack - 1;

        let from_idx = from_stack as usize;
        let to_idx = to_stack as usize;

        for _ in 1..=number_of_crates {
            println!("line :: {} from_idx {} to_idx {} ", line, from_idx, to_idx);
            println!("{:?}", crates[from_idx]);
            println!("{:?}", crates[to_idx]);
            let popped_value = crates[from_idx].pop().unwrap();

            crates[to_idx].push(popped_value);
            println!("RESULT: {:?}", crates[to_idx]);
        }

    }

    crates
}

fn exec_crate_instructions_p2<'a>(instructions: &&str, crates: &'a mut [Vec<char>]) -> &'a mut [Vec<char>] {
    
    for line in instructions.lines() {
        let split_line = line.split(' ');
        let words = split_line.collect::<Vec<&str>>();

        let (number_of_crates, from_stack, to_stack) = parse_instruction_message(&words);
        
        let from_stack = from_stack - 1;
        let to_stack = to_stack - 1;

        let from_idx = from_stack as usize;
        let to_idx = to_stack as usize;

        //for _ in 1..=number_of_crates {
            // for the number of crates we get through pop() we must ensure they're put onto the
            // stack in the right order. for instance, if we're moving 2 from ['A', 'B', 'C'] to a
            // stack of ['D'] then we need to be sure it is ['D', 'B', 'C'] and not ['D', 'C', 'B']
            // basically, preserve the order
            // (vec.iter().map(|v| v.pop()).rev().collect() ?
            let number_of_crates = number_of_crates as usize;
            println!("INSTRUCTION: {}", line);
            println!("len {} number_of_crates {}", crates[from_idx].len(), number_of_crates);
            let split_idx = crates[from_idx].len() - number_of_crates;
            let mut values = crates[from_idx].split_off(split_idx);

            crates[to_idx].append(&mut values);

            println!("splitting {:?}", values);

        //}
    }

    crates
}

fn split_contents_p2(contents: String) {
    let split_contents: Vec<&str> = contents.split("\n\n").collect();
    let (current_stack, instructions) = (
        split_contents.first().unwrap(),
        split_contents.last().unwrap(), 
    );

    let line_length = get_line_length(current_stack);
    let num_crates = get_num_crates(current_stack);

    println!("line length and num crates {} {}", line_length, num_crates);

    let mut crates = get_crates_state(current_stack, &num_crates, &line_length);

    exec_crate_instructions_p2(instructions, crates.as_mut_slice());

    for stack in crates.iter() {
        println!("{:?}", stack.to_owned().pop().unwrap());
    }
}

fn split_contents_p1(contents: String) {
    let split_contents: Vec<&str> = contents.split("\n\n").collect();
    let (current_stack, instructions) = (
        split_contents.first().unwrap(),
        split_contents.last().unwrap(),
    );

    println!("current_stack {}\n\n", current_stack);

    let line_length = get_line_length(current_stack);
    let num_crates = get_num_crates(current_stack);

    let mut crates = get_crates_state(current_stack, &num_crates, &line_length);

    exec_crate_instructions_p1(instructions, crates.as_mut_slice());

    // do the things
    println!("{:#?}", crates);

    for stack in crates.iter() {
        println!("{:?}", stack.to_owned().pop().unwrap());  
    }
}

fn main() {
    let test = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    //let contents = read_file(test);
    let contents = read_file("input.txt");
    split_contents_p2(contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const EXAMPLE2: &str = "[T]     [Q]             [S]        
[R]     [M]             [L] [V] [G]
[D] [V] [V]             [Q] [N] [C]
[H] [T] [S] [C]         [V] [D] [Z]
[Q] [J] [D] [M]     [Z] [C] [M] [F]
[N] [B] [H] [N] [B] [W] [N] [J] [M]
[P] [G] [R] [Z] [Z] [C] [Z] [G] [P]
[B] [W] [N] [P] [D] [V] [G] [L] [T]
 1   2   3   4   5   6   7   8   9 ";

    #[test]
    fn test_read_file() {
        let contents = read_file("input.txt");
        assert_eq!(contents.lines().count(), 512);
    }

    #[test]
    fn test_split_contents_p1() {
        split_contents_p1(String::from(EXAMPLE));
        // split_contents(String::from(EXAMPLE2));
    }
    
    #[test]
    fn test_split_contents_p2() {
        split_contents_p2(String::from(EXAMPLE));
    }
}
