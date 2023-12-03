use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./test01.txt") {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
            } else {
                println!("Error on getting the line content");
            }
        }
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
