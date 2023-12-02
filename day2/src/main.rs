use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum Colors {
    Red(u32),
    Blue(u32),
    Green(u32)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_game(game: &str) -> u32 {
    let mut tokens = game.split_whitespace();
    tokens.next();
    let digit = tokens.next().unwrap();
    digit.parse::<u32>().unwrap()
}

fn parse_turns(turns: &str) -> HashMap::<&str, u32> {
    let parsed_turns = turns.split(";").map(|set| {
        set.split(",").map(|color| {
            let parsed_color = parse_color(color);
            parsed_color
        }).collect::<Vec<Colors>>()
    }).flatten();
    let mut color_counts = HashMap::from(
        [("red", 0), ("blue", 0), ("green", 0)]
    );
    for c in parsed_turns {
        match c {
            Colors::Red(i) => {
                if i > *color_counts.get_mut("red").unwrap() {
                    *color_counts.get_mut("red").unwrap() = i;
                }
            },
            Colors::Blue(i) => {
                if i > *color_counts.get_mut("blue").unwrap() {
                    *color_counts.get_mut("blue").unwrap() = i;
                }
            },
            Colors::Green(i) => {
                if i > *color_counts.get_mut("green").unwrap() {
                    *color_counts.get_mut("green").unwrap() = i;
                }
            },
        };
    }
    color_counts
}

fn parse_color(color: &str) -> Colors {
    let mut entries = color.split_whitespace();
    let digit = entries.next().unwrap();
    let color_name = entries.next().unwrap();
    match color_name {
        "red" => Colors::Red(digit.parse::<u32>().unwrap()),
        "blue" => Colors::Blue(digit.parse::<u32>().unwrap()),
        "green" => Colors::Green(digit.parse::<u32>().unwrap()),
        &_ => panic!("aah!"),
    }
}


fn part1(line: String) -> u32 {
    let mut game_turns: Vec<&str> = line.split(":").collect();
    let turns = game_turns.pop().unwrap();
    let game = parse_game(game_turns.pop().unwrap());
    let color_map = parse_turns(turns);
    if color_map["red"] <= 12 && color_map["blue"] <= 14 && color_map["green"] <= 13 {
        return game;
    }
    0
}

fn part2(line: String) -> u32 {
    let mut game_turns: Vec<&str> = line.split(":").collect();
    let turns = game_turns.pop().unwrap();
    let _  = parse_game(game_turns.pop().unwrap());
    let color_map = parse_turns(turns);
    return color_map["red"]*color_map["blue"]*color_map["green"];
}

fn main() {
    let lines = read_lines("input.txt");
    println!("{}", lines.iter().map(|l| part2(l.clone())).sum::<u32>());
}
