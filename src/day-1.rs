use std::fs;

/// returns an options of pair of the spelled out number and the moved iter
fn check_for_spelled_out_num(line: &[u8], iter: usize) -> Option<(char, usize)> {
    let char = line[iter] as char;

    match char {
        'o' => {
            // one
            if iter + 2 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'n' as u8 && line[iter + 2] == 'e' as u8 {
                return Some(('1', iter + 2));
            }

            None
        },
        't' => {
            // two, three

            // two
            if iter + 2 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'w' as u8 && line[iter + 2] == 'o' as u8 {
                return Some(('2', iter + 2));
            }

            // three
            if iter + 4 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'h' as u8 && line[iter + 2] == 'r' as u8 &&
                line[iter + 3] == 'e' as u8 && line[iter + 4] == 'e' as u8 {

                return Some(('3', iter + 4));
            }

            None
        },
        'f' => {
            // four, five

            if iter + 3 >= line.len() {
                return None;
            }

            // four
            if line[iter + 1] == 'o' as u8 && line[iter + 2] == 'u' as u8 &&
                line[iter + 3] == 'r' as u8 {

                return Some(('4', iter + 3));
            }

            // five
            if line[iter + 1] == 'i' as u8 && line[iter + 2] == 'v' as u8 &&
                line[iter + 3] == 'e' as u8 {

                return Some(('5', iter + 3));
            }

            None
        },
        's' => {
            // six, seven

            // six
            if iter + 2 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'i' as u8 && line[iter + 2] == 'x' as u8 {
                return Some(('6', iter + 2));
            }

            // seven
            if iter + 4 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'e' as u8 && line[iter + 2] == 'v' as u8
                && line[iter + 3] == 'e' as u8 && line[iter + 4] == 'n' as u8 {

                return Some(('7', iter + 4))
            }

            None
        },
        'e' => {
            // eight
            if iter + 4 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'i' as u8 && line[iter + 2] == 'g' as u8
                && line[iter + 3] == 'h' as u8 && line[iter + 4] == 't' as u8 {

                return Some(('8', iter + 4));
            }

            None
        },
        'n' => {
            // nine
            if iter + 3 >= line.len() {
                return None;
            }

            if line[iter + 1] == 'i' as u8 && line[iter + 2] == 'n' as u8
                && line[iter + 3] == 'e' as u8 {

                return Some(('9', iter + 3));
            }

            None
        },
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

            if let Some((n, _)) = check_for_spelled_out_num(line, beg_iter) {
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

            if let Some((n, _)) = check_for_spelled_out_num(line, end_iter as usize) {
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
    let input = fs::read_to_string("../inputs/day1.txt").expect("unable to read file");
    solve(&input);
}
