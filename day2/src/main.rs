use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let filepath = "./input.txt";
	let (mut validcount_p1, mut validcount_p2) = (0,0);
	if let Ok(lines) = read_lines(filepath) {
		for line in lines {
			if let Ok(s) = line {
				// Extraction of data from each line
				let s1: Vec<&str> = s.split(":").collect();
				let s2: Vec<&str> = s1[0].split("-").collect();
				let policy_char = s2[1].chars().last().unwrap();
				let min_count = s2[0].parse::<usize>().unwrap();
				let max_count = s2[1].split(" ").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
				
				// (Part 1) Analysis of each password
				let password = s1[1];
				let c = password.matches(policy_char).count();
				if c >= min_count && c <= max_count {
				    validcount_p1 += 1;
				}

				// (Part 2) Analysis of each password
				// Not sure why chars().nth() is 1-index not 0-index
				if (password.chars().nth(min_count).unwrap() == policy_char && password.chars().nth(max_count).unwrap() != policy_char)
				    || (password.chars().nth(min_count).unwrap() != policy_char && password.chars().nth(max_count).unwrap() == policy_char) {
						validcount_p2 += 1;
				}
			}
		}
	}
	println!("Part 1: {}\nPart 2: {}", validcount_p1, validcount_p2);
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
