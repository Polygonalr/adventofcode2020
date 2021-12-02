use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error;
use std::collections::HashMap;
use regex::Regex;

#[derive(Eq, PartialEq, Hash)]
enum AttrType {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid
}

impl std::str::FromStr for AttrType {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<AttrType, Box<dyn error::Error>> {
        match s {
            "byr" => Ok(AttrType::Byr),
            "iyr" => Ok(AttrType::Iyr),
            "eyr" => Ok(AttrType::Eyr),
            "hgt" => Ok(AttrType::Hgt),
            "hcl" => Ok(AttrType::Hcl),
            "ecl" => Ok(AttrType::Ecl),
            "pid" => Ok(AttrType::Pid),
            "cid" => Ok(AttrType::Cid),
            _ => Err(format!("{} is not valid input!", s).into()),
        }
    }
}

fn check_passport_p2(passport_str: &str) -> bool {
    let attrs: Vec<&str> = passport_str.split(" ").collect();

    // Initialization of the HashMap to record down the attributes satisfied
    let mut attr_check: HashMap<AttrType, bool> = HashMap::new();
    let init_tup_vec = ["byr", "iyr", "eyr", "hgt", "hcl" ,"ecl", "pid"];
    for attr in init_tup_vec.iter() {
        let temp: AttrType = attr.parse().unwrap();
        attr_check.insert(temp, false);
    }

    // Declare all the closures used in validating attributes
    let check_byr = |x: i32| -> bool {
        if x >= 1920 && x <= 2002 {
            return true;
        }
        false
    };
    let check_iyr = |x: i32| -> bool {
        if x >= 2010 && x <= 2020 {
            return true;
        }
        false
    };
    let check_eyr = |x: i32| -> bool {
        if x >= 2020 && x <= 2030 {
            return true;
        }
        false
    };
    let check_hgt = |x: &str| -> bool {
        if !x.ends_with("in") && !x.ends_with("cm") {
            return false;
        }
        let mut chars = x.chars(); // Converts to iterator of chars
        chars.nth_back(1); // remove last 2 chars
        let new_str = chars.as_str();  // Convert back to str
        if let Ok(num_val) = new_str.parse::<i32>() {
            if x.ends_with("cm") && num_val >= 150 && num_val <= 193 {
                return true;
            } else if x.ends_with("in") && num_val >= 59 && num_val <= 76 {
                return true;
            }
        }
        false
    };
    let check_hcl = |x: &str| -> bool {
        let re = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
        return re.is_match(x);
    };
    let check_ecl = |x: &str| -> bool {
        let re = Regex::new(r"^((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))$").unwrap();
        return re.is_match(x);
    };
    let check_pid = |x: &str| -> bool {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        return re.is_match(x);
    };

    // Look through attrs and record down attributes satisfied
    for attr in attrs {
        let attr_str = attr.split(":").collect::<Vec<&str>>();
        if attr_str[0] == ""{
            continue;
        }
        let attr_enum: AttrType = attr_str[0].parse().unwrap();
        let satisfy: bool = match attr_enum {
            AttrType::Byr => check_byr(attr_str[1].parse::<i32>().unwrap()),
            AttrType::Iyr => check_iyr(attr_str[1].parse::<i32>().unwrap()),
            AttrType::Eyr => check_eyr(attr_str[1].parse::<i32>().unwrap()),
            AttrType::Hgt => check_hgt(attr_str[1]),
            AttrType::Hcl => check_hcl(attr_str[1]),
            AttrType::Ecl => check_ecl(attr_str[1]),
            AttrType::Pid => check_pid(attr_str[1]),
            AttrType::Cid => true
        };


        attr_check.insert(attr_enum, satisfy);
    }

    // Look through attr_check and see whether there's any unsatisfied attributes
    let mut satisfied = true;
    for (_, record) in &attr_check{
        if !record {
            satisfied = false;
            break;
        }
    }
    satisfied
}

fn check_passport_p1(passport_str: &str) -> bool {
    let attrs: Vec<&str> = passport_str.split(" ").collect();

    // Initialization of the HashMap to record down the attributes satisfied
    let mut attr_check: HashMap<AttrType, bool> = HashMap::new();
    let init_tup_vec = ["byr", "iyr", "eyr", "hgt", "hcl" ,"ecl", "pid"];
    for attr in init_tup_vec.iter() {
        let temp: AttrType = attr.parse().unwrap();
        attr_check.insert(temp, false);
    }

    // Look through attrs and record down attributes satisfied
    for attr in attrs {
        let attr_str = attr.split(":").collect::<Vec<&str>>()[0];
        if attr_str == ""{
            continue;
        }
        let attr_enum: AttrType = attr_str.parse().unwrap();
        attr_check.insert(attr_enum,  true);
    }

    // Look through attr_check and see whether there's any unsatisfied attributes
    let mut satisfied = true;
    for (_, record) in &attr_check{
        if !record {
            satisfied = false;
            break;
        }
    }
    satisfied
}

fn main() {
    let filepath = "./input.txt";
    let mut passport_str_buf = "".to_owned();
    let (mut counter_p1, mut counter_p2) = (0, 0);
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(s) = line {
                // Build the passport_str_buf if the current line is not empty, check the passport
                // base on the buffer if an empty line is reached
                if s == "" {
                    if check_passport_p1(&passport_str_buf) { 
                        counter_p1 += 1;
                    }
                    if check_passport_p2(&passport_str_buf) {
                        counter_p2 += 1;
                    }
                    passport_str_buf = "".to_owned();
                }

                else {
                    passport_str_buf.push_str(" ");
                    passport_str_buf.push_str(&s);
                }
            }
        }
    }

    // Check the final passport
    if check_passport_p1(&passport_str_buf) {
       counter_p1 += 1;
    }
    if check_passport_p2(&passport_str_buf) {
        counter_p2 += 1;
    }
    println!("Part 1: {}\nPart 2: {}", counter_p1, counter_p2);

}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
