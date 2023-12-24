use std::collections::HashSet;
use std::fs;

fn parse_numbers(numbers: &str) -> HashSet<u8> {
    // assume format is always correct
    numbers.split(" ")
        .filter(|s| !s.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string("inputs/day4.txt").expect("unable to read file");

    let lines = input.split("\n").map(|s| String::from(s)).collect::<Vec<String>>();

    let mut sum = 0;
    for line in lines {
        let (_, numbers_part) = line.split_once(":").unwrap();
        let (winning_numbers, own_numbers) = numbers_part.split_once("|").unwrap();

        let winning_numbers = parse_numbers(winning_numbers);
        let own_numbers = parse_numbers(own_numbers);

        let mut match_count = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                match_count += 1;
            }
        }

        if match_count > 0 {
            sum += 2_i32.pow(match_count - 1);
        }
    }

    println!("{}", sum);
}