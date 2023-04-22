use std::fs;

// check if ranges are contained within either one of another
// first within second: first_start >= second_start && first_end <= second_end
// second within first: second_start >= first_start && second_end <= first_end
fn within_range(first_range: Vec<u32>, second_range: Vec<u32>) -> bool {
    first_range.first() >= second_range.first() && first_range.last() <= second_range.last()
        || second_range.first() >= first_range.first() && second_range.last() <= first_range.last()
}

fn parse_elf_assignments(contents: String) -> (u32, u32) {
    let mut total_containment = 0;
    let mut total_pairs = 0;

    for line in contents.lines() {
        let ranges: Vec<&str> = line.split(',').collect();
        let (first_range, second_range) = (
            ranges.first().unwrap().to_owned(),
            ranges.last().unwrap().to_owned(),
        );
        let (first_range_split, second_range_split): (Vec<&str>, Vec<&str>) = (
            first_range.split('-').collect(),
            second_range.split('-').collect(),
        );

        println!("{:?} {:?} ", first_range_split, second_range_split);

        let (first_range_start, first_range_end, second_range_start, second_range_end) = (
            first_range_split
                .first()
                .unwrap()
                .to_owned()
                .parse::<u32>()
                .unwrap(),
            first_range_split
                .last()
                .unwrap()
                .to_owned()
                .parse::<u32>()
                .unwrap(),
            second_range_split
                .first()
                .unwrap()
                .to_owned()
                .parse::<u32>()
                .unwrap(),
            second_range_split
                .last()
                .unwrap()
                .to_owned()
                .parse::<u32>()
                .unwrap(),
        );

        let first_range = (first_range_start..=first_range_end).collect::<Vec<_>>();
        let second_range = (second_range_start..=second_range_end).collect::<Vec<_>>();

        println!("{:?} {:?}", first_range, second_range);

        if within_range(first_range.clone(), second_range.clone()) {
            total_containment += 1;
        }

        for first in first_range.iter() {
            if second_range.contains(first) {
                total_pairs += 1;
                break;
            }
        }

        // println!("{:?} {:?}", first_range, second_range);
    }

    (total_containment, total_pairs)
}

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

fn main() {
    let contents = read_file("input.txt");
    let (total_containment, total_pairs) = parse_elf_assignments(contents);
    println!("total {} {}", total_containment, total_pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_read_file() {
        let contents = read_file("input.txt");
        assert_eq!(contents.lines().count(), 1000);
    }

    #[test]
    fn test_parse_elf_assignments() {
        let ranges = parse_elf_assignments(String::from(EXAMPLE));
        assert_eq!(ranges, (2, 10));
    }
}
