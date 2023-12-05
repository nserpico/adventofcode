use rayon::prelude::*;
use std::{collections::HashMap, fs::read_to_string};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

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
    (0..seeds.len())
        .into_par_iter()
        .step_by(2)
        .map(|i| {
            (seeds[i]..(seeds[i] + seeds[i + 1]))
                .into_par_iter()
                .map(|k| k)
                .collect::<Vec<u64>>()
        })
        .flatten()
        .collect::<Vec<u64>>()
}

fn main() {
    let whole_content = read("input.txt");
    let chunks: Vec<&str> = whole_content.split("\n\n").collect();
    let seeds = chunks[0].split(" ").collect::<Vec<&str>>()[1..]
        .into_par_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!("read seeds");
    let maps = &chunks[1..];
    let processed_maps = maps
        .into_par_iter()
        .map(|m| process_map(m))
        .collect::<Vec<Vec<Vec<u64>>>>();
    println!("read maps");
    let exp_seeds = expand_seeds(seeds);
    println!("starting!");
    let total = exp_seeds.len();
    println!("{}", total);
    let count = Arc::new(AtomicU64::new(0));
    let result = exp_seeds
        .into_par_iter()
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
            let c = count.fetch_add(1, Ordering::SeqCst);
            if c % 10000 == 0 {
                println!("{:?}/{}", c, total);
            }
            // println!("{}", seed_transformer);
            seed_transformer
        })
        .min();
    println!("{}", result.unwrap());
}
