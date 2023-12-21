use std::fs;

fn parse_line(line: &str) -> Option<(u16, bool)> {
    let (id_part, sets_part) = line.split_once(":")?;

    let (_, id) = id_part.split_at(5);
    let id = id.parse::<u16>().expect("Unable to parse id");

    let sets = sets_part.split(";");
    for set in sets {
        let cubes = set.split(",");
        for cube in cubes {
            let (count, color) = cube.trim().split_once(" ")?;
            let count = count.parse::<i32>().expect("Unable to parse cube number");
            match color.to_ascii_lowercase().as_str() {
                "blue" => {
                    if count > 14 {
                        return Some((id, false));
                    }
                }
                "green" => {
                    if count > 13 {
                        return Some((id, false));
                    }
                }
                "red" => {
                    if count > 12 {
                        return Some((id, false));
                    }
                }
                _ => {}
            }
        }
    }

    Some((id, true))
}

fn solve(input: &String) {
    let line_iter = input.split("\n");
    let mut sum = 0;
    for line in line_iter {
        if let Some((id, possible)) = parse_line(line) {
            if possible { sum += id }
        }
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").expect("unable to read file");
    solve(&input);
}