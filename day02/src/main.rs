use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = false;

fn minimum_number_of_cubes(sets: Vec<(u32, u32, u32)>) -> (u32, u32, u32) {
    let mut min_r: u32 = 0;
    let mut min_g: u32 = 0;
    let mut min_b: u32 = 0;
    for (r, g, b) in &sets {
        min_r = max(*r, min_r);
        min_g = max(*g, min_g);
        min_b = max(*b, min_b);
    }

    return (min_r, min_g, min_b);
}

fn evaluate_feasibility(r: &u32, g: &u32, b: &u32) -> bool {
    return r <= &12 && g <= &13 && b <= &14;
}

fn parse_set(line: &str) -> (u32, u32, u32) {
    // line = "2 red, 3 blue"
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for cube_str in line.split(", ").collect::<Vec<&str>>() {
        let cube_str_split: Vec<&str> = cube_str.split(" ").collect();
        let number: u32 = cube_str_split[0].parse().unwrap();
        match cube_str_split[1] {
            "red" => r = number,
            "green" => g = number,
            "blue" => b = number,
            _ => (),
        }
    }
    return (r, g, b);
}

fn parse_game_data(line: &String) -> (u32, Vec<(u32, u32, u32)>) {
    let split: Vec<&str> = line.split(": ").collect();
    let id: u32 = split[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let sets = split[1].split("; ").map(|set| parse_set(set)).collect();

    return (id, sets);
}

fn main() {
    let mut sum_id = 0;
    let mut sum_power_min = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(content) = line {
                if DEBUG {
                    println!("{}", content);
                }
                let (id, sets) = parse_game_data(&content);
                if DEBUG {
                    println!("[debug] Id: {}", id);
                    for (r, g, b) in &sets {
                        println!("[debug] (r, g, b) = ({}, {}, {})", r, g, b);
                    }
                }
                let mut possible = true;
                for (r, g, b) in &sets {
                    possible = possible && evaluate_feasibility(r, g, b);
                }
                if possible {
                    sum_id += id;
                }

                let (min_r, min_g, min_b) = minimum_number_of_cubes(sets);
                sum_power_min += min_r * min_g * min_b;
            } else {
                println!("Error on getting the line content");
            }
        }
        println!("Sum ID Q1: {}", sum_id);
        println!("Sum power of the minimal set Q2: {}", sum_power_min);
    } else {
        println!("File does not exists");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
