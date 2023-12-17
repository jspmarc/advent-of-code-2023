use std::fs;

fn try_parse(line: &[u8], iter: usize, return_val: char, spelled_out: &'static [u8]) -> Option<char> {
    let needed_additional_len = spelled_out.len() - 1;
    if iter + needed_additional_len >= line.len() {
        return None;
    }

    let mut iter = iter + 1;
    let mut i = 1;
    let mut matched = true;
    while i < spelled_out.len() {
        if spelled_out[i] != line[iter] {
            matched = false;
            break;
        }

        i += 1;
        iter += 1;
    }

    if matched {
        return Some(return_val);
    }

    None
}

/// returns an options of pair of the spelled out number and the moved iter
fn check_for_spelled_out_num(line: &[u8], iter: usize) -> Option<char> {
    let char = line[iter] as char;

    match char {
        // one
        'o' => try_parse(line, iter, '1', "one".as_bytes()),
        // two, three
        't' => if let Some(r) = try_parse(line, iter, '2', "two".as_bytes()) {
            Some(r)
        } else {
            try_parse(line, iter, '3', "three".as_bytes())
        }
        // four, five
        'f' => if let Some(r) = try_parse(line, iter, '4', "four".as_bytes()) {
            Some(r)
        } else {
            try_parse(line, iter, '5', "five".as_bytes())
        }
        // six, seven
        's' => if let Some(r) = try_parse(line, iter, '6', "six".as_bytes()) {
            Some(r)
        } else {
            try_parse(line, iter, '7', "seven".as_bytes())
        }
        // eight
        'e' => try_parse(line, iter, '8', "eight".as_bytes()),
        // nine
        'n' => try_parse(line, iter, '9', "nine".as_bytes()),
        _ => None
    }
}

fn solve(input: &String) {
    let mut sum = 0;

    let lines_iter = input.split_ascii_whitespace();
    for line in lines_iter {
        let line = line.as_bytes();

        let mut missing_number = String::new();

        // first digit
        let mut beg_iter = 0_usize;
        while beg_iter < line.len() {
            let char = line[beg_iter] as char;
            if char >= '0' && char <= '9' {
                missing_number.push(char);
                break;
            }

            if let Some(n) = check_for_spelled_out_num(line, beg_iter) {
                missing_number.push(n);
                break;
            }

            beg_iter += 1;
        }

        // second digit
        let mut end_iter = (line.len() - 1) as isize;
        while end_iter >= 0_isize {
            let char = line[end_iter as usize] as char;
            if char >= '0' && char <= '9' {
                missing_number.push(char);
                break;
            }

            if let Some(n) = check_for_spelled_out_num(line, end_iter as usize) {
                missing_number.push(n);
                break;
            }

            end_iter -= 1;
        }

        sum += missing_number.parse::<i32>().expect("unable to parse to a number");
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").expect("unable to read file");
    solve(&input);
}
