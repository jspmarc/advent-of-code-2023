use std::fs;

fn parse_line(line: &str, is_part_1: bool) -> Option<(i32, bool)> {
    let (id_part, sets_part) = line.split_once(":")?;

    let (_, id) = id_part.split_at(5);

    // for part 1
    let id = id.parse::<i32>().expect("Unable to parse id");

    // for part 2
    let mut part_2_product = 0;
    let (mut red_max, mut blue_max, mut green_max) = (i32::MIN, i32::MIN, i32::MIN);

    let sets = sets_part.split(";");
    for set in sets {
        let cubes = set.split(",");
        for cube in cubes {
            let (count, color) = cube.trim().split_once(" ")?;
            let count = count.parse::<i32>().expect("Unable to parse cube number");
            match color.to_ascii_lowercase().as_str() {
                "blue" => {
                    if is_part_1 && count > 14 {
                        return Some((id, false));
                    } else if !is_part_1 && count > blue_max {
                        blue_max = count;
                    }
                }
                "green" => {
                    if is_part_1 && count > 13 {
                        return Some((id, false));
                    } else if !is_part_1 && count > green_max {
                        green_max = count;
                    }
                }
                "red" => {
                    if is_part_1 && count > 12 {
                        return Some((id, false));
                    } else if !is_part_1 && count > red_max {
                        red_max = count;
                    }
                }
                _ => {}
            }
        }
    }

    if !is_part_1 {
        part_2_product = red_max * blue_max * green_max;
    }

    if is_part_1 {
        Some((id, true))
    } else {
        Some((part_2_product, true))
    }
}

fn solve(input: &String, is_part_1: bool) {
    let line_iter = input.split("\n");
    let mut sum = 0;
    for line in line_iter {
        if let Some((res, possible)) = parse_line(line, is_part_1) {
            if possible { sum += res }
        }
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").expect("unable to read file");
    solve(&input, true);
    solve(&input, false);
}