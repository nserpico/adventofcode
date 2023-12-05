use rayon::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

fn read(filename: &str) -> String {
    read_to_string(filename).unwrap()
}
// dest source range

fn do_map(seed: u64, digits: &Vec<u64>) -> u64 {
    if seed >= digits[1] && seed < digits[1] + digits[2] {
        return seed - digits[1] + digits[0];
    }
    0
}

fn process_map(map_data: &str) -> Vec<Vec<u64>> {
    let lines: Vec<&str> = map_data.split("\n").collect();
    lines[1..]
        .into_iter()
        .map(|l| {
            l.split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn expand_seeds(seeds: Vec<u64>) -> Vec<u64> {
    let mut i = 0;
    let mut expanded = vec![];
    while i < seeds.len() - 1 {
        for k in seeds[i]..(seeds[i] + seeds[i + 1]) {
            expanded.push(k);
        }
        i += 2;
    }
    expanded
}

fn main() {
    let whole_content = read("input.txt");
    let chunks: Vec<&str> = whole_content.split("\n\n").collect();
    let seeds = chunks[0].split(" ").collect::<Vec<&str>>()[1..]
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let maps = &chunks[1..];
    let processed_maps = maps
        .into_iter()
        .map(|m| process_map(m))
        .collect::<Vec<Vec<Vec<u64>>>>();
    let mut total = 0;
    let result = expand_seeds(seeds)
        .into_iter()
        .map(|seed| {
            let mut seed_transformer = seed;
            for section_maps in &processed_maps {
                for m in section_maps {
                    let res = do_map(seed_transformer, &m);
                    if res != 0 {
                        seed_transformer = res;
                        break;
                    }
                }
            }
            total += 1;
            if total % 1000000 == 0 {
                println!("{}", total)
            }
            seed_transformer
        })
        .min();
    println!("{}", result.unwrap());
}
