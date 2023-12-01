use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn read_lines_process(filename: &str) -> Vec<String> {
    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result = Vec::new();
    let mut content = read_to_string(filename).unwrap();
    for line in content.lines() {
        let mut work_line = String::from(line);
        let mut i = 0;
        while i < work_line.len() {
            for (idx, word) in numbers.iter().enumerate() {
                if work_line[i..].starts_with(word) {
                    work_line.insert_str(i + 1, &(idx + 1).to_string());
                    println!("{}", work_line);
                    i += 1;
                    break;
                }
            }
            i += 1;
        }
        result.push(work_line);
    }
    result
}

fn do_it(lines: Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let combined = match nums.len() {
            0 => 0,
            n => {
                let first = nums[0];
                let last = nums[n - 1];
                format!("{first}{last}").parse::<u32>().unwrap()
            }
        };
        sum += combined;
    }
    println!("{}", sum)
}

fn part1() {
    let lines: Vec<String> = read_lines("input.txt");
    do_it(lines);
}

fn part2() {
    let lines: Vec<String> = read_lines_process("input.txt");
    do_it(lines)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    // Code block to measure.
    {
        part2()
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
