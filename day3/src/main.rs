use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn is_by_symbol(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut min_r = 0;
    if row != 0 {
        min_r = row-1;
    }
    let mut min_c = 0;
    if col != 0 {
        min_c = col-1;
    }
    for r in (min_r)..(row+2){
        if r >= grid.len() {
            continue;
        }
        for c in (min_c)..(col+2) {
            if c >= grid.len() {
                continue;
            }
            if !grid[r][c].is_digit(10) && grid[r][c] != '.' {
                return true
            }
        }
    }
    false
}

fn by_which_symbols(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut min_r = 0;
    let mut symbols: Vec<(usize, usize)> = vec![];
    if row != 0 {
        min_r = row-1;
    }
    let mut min_c = 0;
    if col != 0 {
        min_c = col-1;
    }
    for r in (min_r)..(row+2){
        if r >= grid.len() {
            continue;
        }
        for c in (min_c)..(col+2) {
            if c >= grid.len() {
                continue;
            }
            if grid[r][c] == '*' {
                symbols.push((r,c));
            }
        }
    }
    symbols
}

fn part1(){
    let lines = read_lines("input.txt");
    let grid: Vec<Vec<char>> = lines.iter().map(|line: &String| {
        line.chars().map(|c| {
            c
        }).collect()
    }).collect();

    let mut r = 0;
    let mut c = 0;
    let mut sum = 0;
    while r < grid.len() {
        let row = &grid[r];
        while c < row.len() {
            let mut whole_num = String::new();
            let mut by_symbol = false;
            while c < grid.len() && row[c].is_digit(10) {
                whole_num.push(row[c]);
                if is_by_symbol(&grid, r, c) {
                    by_symbol = true;
                }
                c += 1;
            }
            if by_symbol {
                println!("{}", whole_num);
                sum += whole_num.parse::<u32>().unwrap();
            }
            c += 1;
        }
        c = 0;
        r += 1;
    }

    println!("{:?}", sum);
}
fn part2(){
    let mut map: HashMap<(usize,usize), Vec<u32>> = HashMap::new();
    let lines = read_lines("input.txt");
    let grid: Vec<Vec<char>> = lines.iter().map(|line: &String| {
        line.chars().map(|c| {
            c
        }).collect()
    }).collect();

    let mut r = 0;
    let mut c = 0;
    while r < grid.len() {
        let row = &grid[r];
        while c < row.len() {
            let mut whole_num = String::new();
            let mut all_cords: Vec<(usize, usize)> = vec![];
            while c < grid.len() && row[c].is_digit(10) {
                whole_num.push(row[c]);
                let mut by_symbols = by_which_symbols(&grid, r, c);
                all_cords.append(&mut by_symbols);
                c += 1;
            }
            let set: HashSet<_> = all_cords.drain(..).collect(); // dedup
            all_cords.extend(set.into_iter());
            if all_cords.len() > 0 {
                for coord in all_cords {
                    map.entry(coord).or_insert(vec![]).push(whole_num.parse::<u32>().unwrap())

                }
            }
            c += 1;
        }
        c = 0;
        r += 1;
    }
    let mut sum = 0;
    for (_, v) in map {
        if v.len() == 2 {
            println!("{:?}", v);
            sum += v[0]*v[1];
        }
    }
    println!("{}",sum);

}
fn main() {
    part2()
}
