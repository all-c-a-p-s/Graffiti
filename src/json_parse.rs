use std::fs::File;
use std::io::prelude::*;

const PATH: &str = "json/2016.json";

const LAST_CLIMB_LINE_NUMBER: usize = 5143741;
const FIRST_CLIMB_LINE_NUMBER: usize = 3;

pub fn read_from_file() -> Result<String, &'static str> {
    let mut file = match File::open(String::from(PATH)) {
        Ok(f) => f,
        Err(_) => return Err("failed to open file"),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => return Err("failed to read file to string"),
    };
    Ok(contents)
}

pub fn get_lines() -> Result<Vec<String>, &'static str> {
    match read_from_file() {
        Ok(s) => Ok(s.lines().into_iter().map(|s| String::from(s)).collect::<Vec<String>>()),
        Err(e) => return Err(e),
    }
}

pub fn next_climb(line_num: usize, lines: &Vec<String>) -> (Option<Vec<String>>, usize) {
    if line_num >= LAST_CLIMB_LINE_NUMBER {
        return(None, line_num);
    }
    let mut bracket_count = 0;
    let mut opened: bool = false;
    let mut new_climb: Vec<String> = Vec::new();
    for line in line_num+1..=LAST_CLIMB_LINE_NUMBER {
        new_climb.push(lines[line].clone());
        for char in 0..lines[line].len() {
            match lines[line].chars().collect::<Vec<char>>()[char] {
                '{' => {
                    bracket_count += 1;
                    opened = true;
                }
                '}' => bracket_count -= 1,
                _ => {},
            }
            if bracket_count == 0 && opened {
                return (Some(new_climb), line);
            }
        }
    }
    (None, LAST_CLIMB_LINE_NUMBER)
}

pub fn get_climbs() -> Result<Vec<Vec<String>>, &'static str> {
    let lines = match get_lines() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    println!("read lines from file");

    let mut climbs_list: Vec<Vec<String>> = Vec::new();
    let mut line_num = FIRST_CLIMB_LINE_NUMBER - 1;
    let mut next: Option<Vec<String>> = Some(Vec::new());
    while next.is_some() {
        (next, line_num) = next_climb(line_num, &lines);
        climbs_list.push(next.clone().expect("tried to append None climb to climb vec"));
    }

    println!("read in climbs list");
    
    Ok(climbs_list)
}


