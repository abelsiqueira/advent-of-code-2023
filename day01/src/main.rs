use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = true;

fn trebuchet_calibration(line: &String) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut set_first_digit = false;
    for char in line.chars() {
        if char.is_numeric() {
            // If the first_digit is -1, then we haven't set it yet
            if !set_first_digit {
                first_digit = char.to_digit(10).unwrap();
                set_first_digit = true;
            }
            // The last digit will always be updated.
            last_digit = char.to_digit(10).unwrap();
        }
    }
    let output = first_digit * 10 + last_digit;
    return output;
}

fn trebuchet_calibration_with_twist(line: &String) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut set_first_digit = false;

    for (idx, char) in line.chars().enumerate() {
        let mut converted_value = 0;
        let mut found_digit = false;
        if char.is_numeric() {
            converted_value = char.to_digit(10).unwrap();
            found_digit = true;
        } else {
            let line_length = line.len();
            // check next few chars for match with words
            if idx + 2 < line_length {
                let word = &line[idx..idx + 3];
                found_digit = true;
                match word {
                    "one" => converted_value = 1,
                    "two" => converted_value = 2,
                    "six" => converted_value = 6,
                    _ => found_digit = false,
                }
                if DEBUG {
                    println!(
                        "[debug] word: {}, value: {}, found: {}",
                        word, converted_value, found_digit
                    );
                }
            }
            if !found_digit && idx + 3 < line_length {
                let word = &line[idx..idx + 4];
                found_digit = true;
                match word {
                    "zero" => converted_value = 0,
                    "four" => converted_value = 4,
                    "five" => converted_value = 5,
                    "nine" => converted_value = 9,
                    _ => found_digit = false,
                }
                if DEBUG {
                    println!(
                        "[debug] word: {}, value: {}, found: {}",
                        word, converted_value, found_digit
                    );
                }
            }
            if !found_digit && idx + 4 < line_length {
                let word = &line[idx..idx + 5];
                found_digit = true;
                match word {
                    "three" => converted_value = 3,
                    "seven" => converted_value = 7,
                    "eight" => converted_value = 8,
                    _ => found_digit = false,
                }
                if DEBUG {
                    println!(
                        "[debug] word: {}, value: {}, found: {}",
                        word, converted_value, found_digit
                    );
                }
            }
        }
        if found_digit {
            if !set_first_digit {
                first_digit = converted_value;
                set_first_digit = true;
                if DEBUG {
                    println!("[debug] first digit is being updated to {}", first_digit);
                }
            }
            // The last digit will always be updated.
            last_digit = converted_value;
        }
    }
    let output = first_digit * 10 + last_digit;
    if DEBUG {
        println!("[debug] output: {output}");
    }
    return output;
}

fn alternative1(line: &String) -> u32 {
    let something: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
    let value =
        something[0].to_digit(10).unwrap() * 10 + something.last().unwrap().to_digit(10).unwrap();

    if DEBUG {
        println!("[debug] value: {}", value);
    }
    return value;
}

fn alternative2(line: &String) -> u32 {
    let something = &line
        .replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    if DEBUG {
        println!("[debug] replaced: {}", something);
    }
    return alternative1(something);
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum1 = 0;
        let mut sum2 = 0;

        for line in lines {
            if let Ok(content) = line {
                if DEBUG {
                    println!("[debug] line content: {}", content);
                }
                // sum1 = sum1 + trebuchet_calibration(&content);
                // sum2 = sum2 + trebuchet_calibration_with_twist(&content);
                sum1 = sum1 + alternative1(&content);
                sum2 = sum2 + alternative2(&content);
            } else {
                println!("Error on getting the line content");
            }
        }
        println!("sum of calibration values: {}", sum1);
        println!("sum of calibration values with a twist: {}", sum2);
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
