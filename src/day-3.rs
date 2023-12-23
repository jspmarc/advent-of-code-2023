use std::fs;

// assume a symbol is any non-dot (ASCII 0x2E) and non-alphabetical (a-z, A-Z) character
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

fn validate_range(line: &Vec<char>, idx_range: (usize, usize)) -> bool {
    // inclusive
    let start_idx = idx_range.0;
    // exclusive
    let end_idx = idx_range.1;

    let mut i = start_idx;
    while i < end_idx {
        let char = line[i];
        if !char.is_ascii_digit() && char != '.' {
            return true;
        }
        i += 1;
    }

    false
}

fn validate_side_perimeter(line_prev: Option<&Vec<char>>, line_curr: &Vec<char>, line_next: Option<&Vec<char>>, perimeter_idx: usize) -> bool {
    let perimeter_char = line_curr[perimeter_idx];
    if !perimeter_char.is_ascii_digit() && perimeter_char != '.' {
        return true;
    }

    if let Some(line_prev) = line_prev {
        let prev_line_perimeter_char = line_prev[perimeter_idx];
        if !prev_line_perimeter_char.is_ascii_digit() && prev_line_perimeter_char != '.' {
            return true;
        }
    }

    if let Some(line_next) = line_next {
        let next_line_perimeter_char = line_next[perimeter_idx];
        if !next_line_perimeter_char.is_ascii_digit() && next_line_perimeter_char != '.' {
            return true;
        }
    }

    false
}

fn is_number_valid(line_prev: Option<&Vec<char>>, line_curr: &Vec<char>, line_next: Option<&Vec<char>>, idx_range: (usize, usize)) -> bool {
    // inclusive
    let start_idx = idx_range.0;
    // exclusive
    let end_idx = idx_range.1;

    // imagine a pattern like this:
    // a...b
    // c123d
    // e...f
    // where alphabet characters *could* be a symbol.
    // we first check for the cells of a, b, c, d, e, and f (we will call this the "side perimeter")
    // then we check the range between a and b
    // then between e and f
    // if from all of the checks none contain a symbol, then number is not valid.

    // check a, c, e cells
    if start_idx != 0 && validate_side_perimeter(line_prev, line_curr, line_next, start_idx - 1) {
        return true;
    }

    // check for b, d, f cells
    if end_idx < line_curr.len() && validate_side_perimeter(line_prev, line_curr, line_next, end_idx) {
        return true;
    }

    if let Some(line_prev) = line_prev {
        if validate_range(line_prev, idx_range) {
            return true;
        }
    }

    if let Some(line_next) = line_next {
        if validate_range(line_next, idx_range) {
            return true;
        }
    }

    false
}

fn search_for_numbers(line_prev: Option<&Vec<char>>, line_curr: &Vec<char>, line_next: Option<&Vec<char>>) -> Vec<i32> {
    let mut valid_numbers = vec![];
    let mut i = 0;
    while i < line_curr.len() {
        let char = line_curr[i];
        if !char.is_ascii_digit() {
            i += 1;
            continue;
        }

        let (number, idx_range) = get_number_and_index_range(&line_curr, &mut i);
        if is_number_valid(line_prev, line_curr, line_next, idx_range) {
            valid_numbers.push(number);
        }
    }

    valid_numbers
}

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").expect("unable to read file");

    let lines = input.split("\n").map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        let line_prev = if (i as i32 - 1) >= 0 {
            lines.get(i - 1)
        } else {
            None
        };
        let line_next = lines.get(i + 1);
        let valid_numbers = search_for_numbers(line_prev, line, line_next);
        sum += valid_numbers.iter().sum::<i32>();
    }

    println!("{}", sum);
}