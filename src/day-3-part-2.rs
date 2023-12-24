use std::collections::HashMap;
use std::fs;

// assume the length of each line is the same

/// 1. get the number
/// 2. move the iterator forward to after the number
/// 3. index range (inclusive in start, exclusive in end)
fn get_number_and_index_range(chars: &Vec<char>, i: &mut usize) -> (i32, (usize, usize)) {
    let mut number = String::from(chars[*i]);
    let start = *i;
    *i = *i + 1;

    while *i < chars.len() && chars[*i].is_ascii_digit() {
        number.push(chars[*i]);
        *i = *i + 1;
    }

    let end = *i;
    (number.parse().expect("Unable to parse number"), (start, end))
}

fn search_for_numbers_in_line(line: &Vec<char>, numbers: &mut HashMap<(usize, usize), i32>) {
    let mut i = 0;

    while i < line.len() {
        let char = line[i];
        if !char.is_ascii_digit() {
            i += 1;
            continue;
        }

        let (number, (start_idx, end_idx)) = get_number_and_index_range(&line, &mut i);
        numbers.insert((start_idx, end_idx), number);
        i += 1;
    }
}

fn process_asterisk((key, value): (&(usize, usize), &i32), i : usize, matched_numbers: &mut Vec::<i32>) {
    let start_idx = key.0;
    let end_idx = key.1;

    if end_idx == i || start_idx == i + 1 || (start_idx <= i && i < end_idx) {
        matched_numbers.push(*value);
    }
}

fn search_for_gears_in_line(lines: &Vec<Vec<char>>, curr_line_idx: usize, numbers: &Vec::<HashMap<(usize, usize), i32>>) -> i32 {
    let line = &lines[curr_line_idx];
    let line_numbers = &numbers[curr_line_idx];
    let line_numbers_prev = if (curr_line_idx as i32 - 1) >= 0 {
        numbers.get(curr_line_idx - 1)
    } else {
        None
    };
    let line_numbers_next = numbers.get(curr_line_idx + 1);

    let mut i = 0;
    let mut gears = Vec::new();

    while i < line.len() {
        let char = line[i];
        if char != '*' {
            i += 1;
            continue;
        }
        let mut matched_numbers = Vec::with_capacity(2);
        line_numbers.iter().for_each(|kv| process_asterisk(kv, i, &mut matched_numbers));
        if let Some(line_numbers_prev) = line_numbers_prev {
            line_numbers_prev.iter().for_each(|kv| process_asterisk(kv, i, &mut matched_numbers));
        }
        if let Some(line_numbers_next) = line_numbers_next {
            line_numbers_next.iter().for_each(|kv| process_asterisk(kv, i, &mut matched_numbers));
        }

        if matched_numbers.len() == 2 {
            gears.push(matched_numbers[0] * matched_numbers[1]);
        }

        i += 1;
    }

    gears.iter().sum()
}

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").expect("unable to read file");

    let lines = input.split("\n").map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    // key is line idx, start idx, end idx
    // let mut numbers = Vec::<HashMap<(usize, usize), i32>>::with_capacity(lines.len());
    let mut numbers = vec![HashMap::new(); lines.len()];
    for i in 0..lines.len() {
        search_for_numbers_in_line(&lines[i], &mut numbers[i]);
    }

    let mut sum = 0;
    for i in 0..lines.len() {
        sum += search_for_gears_in_line(&lines, i, &numbers);
    }

    println!("{}", sum);
}