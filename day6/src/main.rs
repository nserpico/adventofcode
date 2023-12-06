// f(t) = (duration-t)t = t*duration - t^2
// distance covered = t(duration-t)
// solve t(d-t) > x
use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve(time: u64, record: u64) -> Vec<u64> {
    (1..time)
        .filter_map(|t| {
            if t * time - t * t > record {
                return Some(t);
            }
            None
        })
        .collect::<Vec<u64>>()
}

// fn solve(time: u64, record: u64) -> u64 {
//     if time - 2 * t = (time - record) / 2 {
//         return Some(t);
//     }
//     None
// }

fn part1() {
    let lines = read_lines("input.txt");
    let time = &lines[0];
    let records = &lines[1];

    let parsed_times = time
        .split(" ")
        .into_iter()
        .filter_map(|x| match x.parse::<u64>() {
            Ok(d) => Some(d),
            Err(e) => None,
        })
        .collect::<Vec<u64>>();

    let parsed_records = records
        .split(" ")
        .into_iter()
        .filter_map(|x| match x.parse::<u64>() {
            Ok(d) => Some(d),
            Err(e) => None,
        })
        .collect::<Vec<u64>>();

    println!("{:?}", parsed_times);
    println!("{:?}", parsed_records);
    println!("Hello, world!");

    let all = parsed_times
        .into_iter()
        .zip(parsed_records.into_iter())
        .map(move |(time, record)| solve(time, record))
        .map(|v| v.len())
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{:?}", all);
}

fn part2() {
    let lines = read_lines("input.txt");
    let time = &lines[0];
    let records = &lines[1];

    let parsed_times = time.replace(" ", "").split(":").collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .unwrap();
    let parsed_records = records.replace(" ", "").split(":").collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .unwrap();

    println!("{:?}", parsed_times);
    println!("{:?}", parsed_records);
    println!("Hello, world!");
    println!("{:?}", solve(parsed_times, parsed_records).len());
}

fn main() {
    part2();
}
