use std::{fs, collections::{HashSet, HashMap}};
use regex::Regex;


fn parse_lines(contents: String) {

    let mut table: HashMap<String, u32> = HashMap::new();
    let mut current_dir = String::from("");

    for line in contents.lines() {

        let split_line = line.split(' ').into_iter().collect::<Vec<&str>>();
        let (first, second, third) = ( 
            split_line.get(0).unwrap(), 
            split_line.get(1).unwrap(), 
            match split_line.get(2) {
                Some(&x) => x,
                None => ""
            }
        );

        if second == &"cd" && first != &"." && first != &".." {
            current_dir = format!("{}{}", current_dir, third);
        } 

        let re = Regex::new(r"(?P<size>[0-1]+)").unwrap();

        if re.is_match(first) {

            let caps = re.captures(first).unwrap();
            let size = &caps["size"];

            println!("got size {}", size);

            let size: u32 = String::from(size).parse().expect("wanted a number");

            if table.contains_key(current_dir.as_str()) {
                let existing = table.get(current_dir.as_str());

                table.insert(current_dir.to_owned(), existing.unwrap() + size);
            } else {
                table.insert(current_dir.to_owned(), size.clone());
            }
        }
    }

    println!("table is {:?}", table);
}

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn main() {
    let test_contents = read_file("example.txt");
    parse_lines(test_contents);
}
