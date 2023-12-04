use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_line(line: &String) -> u32 {
    let game = line.split(": ").collect::<Vec<&str>>()[1];
    let parts = game.split("|").collect::<Vec<&str>>();
    let winning = parts[0].trim();
    let score = parts[1].trim();
    println!("{:?} {:?}", winning, score);
    let split_winning = winning
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    let split_score = score
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    split_winning
        .intersection(&split_score)
        .count()
        .try_into()
        .unwrap()
}

fn part1() {
    let lines = read_lines("input.txt");
    let score = lines
        .iter()
        .map(|line| {
            let total = parse_line(line);
            if total > 0 {
                return u32::pow(2, total - 1);
            }
            0
        })
        .sum::<u32>();
    println!("{}", score);
}

fn part2() {
    let lines = read_lines("input.txt");

    let scores = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<u32>>();
    let mut copies = vec![1; scores.len()];
    let mut i = 0;
    while i < scores.len() {
        let card = scores[i];
        let mut additional = card;
        let mut j = i + 1;
        while additional > 0 && j < copies.len() {
            copies[j] += 1 * copies[i];
            j += 1;
            additional -= 1;
        }
        i += 1;
    }

    let mut iter = 0;
    let mut final_sum = 0;
    while iter < scores.len() {
        final_sum += copies[iter];
        iter += 1;
    }
    println!("{}", final_sum);
}

fn main() {
    part2();
}
