use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_trees_encountered(x_gradient: usize, y_gradient: usize) -> usize { 
    let filepath = "./input.txt";
    let (mut x_offset, mut y_index, mut counter) = (x_gradient-1,0,0);
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            x_offset += 1;
            if x_offset < x_gradient {
                continue;
            }
            else {
                x_offset = 0;
            }
            if let Ok(s) = line {
                if s.chars().nth(y_index).unwrap() == '#' {
                    counter += 1;
                }
                y_index += y_gradient;
                if y_index >= s.len() {
                    y_index -= s.len();
                }
            }
        }
    }
    counter
}

fn main() {
    let part2ans = count_trees_encountered(1,1) * count_trees_encountered(1,3) * count_trees_encountered(1,5) * count_trees_encountered(1,7) * count_trees_encountered(2,1);
    println!("Part 1: {}\nPart 2: {}", count_trees_encountered(1,3), part2ans);

}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
