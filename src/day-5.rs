use std::fs;

fn get_seed_location(seed: u64, maps: &Vec<Vec<[u64; 3]>>) -> u64 {
    let mut cur_val = seed;

    for map in maps {
        for vals in map {
            let dest_range_start = vals[0];
            let src_range_start = vals[1];
            let range = vals[2];

            if src_range_start <= cur_val && cur_val < src_range_start + range {
                cur_val = dest_range_start + (cur_val - src_range_start);
                break;
            }
        }
    }

    cur_val
}

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").expect("unable to read file");
    let mut lines = input.split('\n');

    let seeds: Vec<u64> = lines.next().unwrap()
        .split_once(' ').unwrap()
        .1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps = Vec::with_capacity(7);
    let mut map: Vec<[u64; 3]> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if !line.as_bytes()[0].is_ascii_digit() {
            if !map.is_empty() {
                maps.push(map);
                map = Vec::new();
            }
            continue;
        }

        map.push(line.split(' ' )
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap());
    }

    if !map.is_empty() {
        maps.push(map);
    }

    let mut min_loc = u64::MAX;
    for seed in &seeds {
        let loc = get_seed_location(*seed, &maps);
        if loc < min_loc {
            min_loc = loc;
        }
    }

    println!("{}", min_loc);
}