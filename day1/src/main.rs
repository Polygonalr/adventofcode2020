use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filepath = "./input.txt";
    let mut vec = Vec::<u16>::new();
    if let Ok(lines) = read_lines(filepath){
        for line in lines {
            if let Ok(s) = line{
                let temp = s.parse::<u16>().unwrap();
                vec.push(temp);
            }
        }
    }

    // First part solution
    let mut veciter = vec.iter();
    while let Some(number) = veciter.next() {
        let diff = 2020u16 - number;
        if vec.iter().any(|&b| b == diff) {
            println!("Part 1: {}", diff as u128 * *number as u128);
            break;
        }
    }

    // Second part solution, naive solution of O(n^3)
    let mut veciter = vec.iter();
    while let Some(number) = veciter.next() {
        let mut vecjter = vec.iter();
        let mut breakloop = false;
        while let Some(number2) = vecjter.next() {
            if number2 == number || 2020u16 < number + number2 {
                continue;
            }
            let diff = 2020u16 - number - number2;
            if vec.iter().any(|&b| b == diff) {
                println!("Part 2: {}", diff as u128 * *number as u128 * *number2 as u128);
                breakloop = true;
                break;
            }
        }
        if breakloop {
            break;
        }
    }
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
