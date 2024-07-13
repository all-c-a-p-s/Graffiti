use std::fs::File;
use std::io::prelude::*;

use crate::climb::*;

const PATH: &str = "json/test.json";

const LAST_CLIMB_LINE_NUMBER: usize = 157;
const FIRST_CLIMB_LINE_NUMBER: usize = 3;
// change back to 5143741 for non-test version

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
        Ok(s) => Ok(s
            .lines()
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<String>>()),
        Err(e) => return Err(e),
    }
}

pub fn next_climb(line_num: usize, lines: &Vec<String>) -> (Option<Vec<String>>, usize) {
    if line_num >= LAST_CLIMB_LINE_NUMBER {
        return (None, line_num);
    }
    let mut bracket_count = 0;
    let mut opened: bool = false;
    let mut new_climb: Vec<String> = Vec::new();
    for line in line_num + 1..=LAST_CLIMB_LINE_NUMBER {
        new_climb.push(lines[line].clone());
        for char in 0..lines[line].len() {
            match lines[line].chars().collect::<Vec<char>>()[char] {
                '{' => {
                    bracket_count += 1;
                    opened = true;
                }
                '}' => bracket_count -= 1,
                _ => {}
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

    #[allow(unused_assignments)]
    let mut next: Option<Vec<String>> = Some(Vec::new());
    loop {
        (next, line_num) = next_climb(line_num, &lines);
        if next.is_none() {
            break;
        }
        climbs_list.push(
            next.clone()
                .expect("tried to append None climb to climb vec"),
        );
    }

    println!("read in climbs list");

    Ok(climbs_list)
}

pub fn find(target: String, lines: &Vec<String>) -> Option<usize> {
    //find target substring in a vec of lines
    let trimmed = lines
        .iter()
        .map(|l| String::from(l.trim()))
        .collect::<Vec<String>>();
    for line_num in 0..trimmed.len() {
        let line = &trimmed[line_num];
        if line.len() < target.len() {
            continue;
        }
        let (mut start, mut end) = (0, target.len() - 1);
        while end < line.len() {
            if line[start..=end] == *target.as_str() {
                return Some(line_num);
            }
            (start, end) = (start + 1, end + 1);
        }
    }
    None
}

// TODO: read in moves in sequence + update moves struct to record move sequence
pub fn parse_climb(lines: Vec<String>) -> Route {
    let grade_line =
        &lines[find(String::from(r#""grade":"#), &lines).expect("failed to find grade")];
    let grade = Grade::from_string(String::from(
        grade_line
            .split_ascii_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>()[1]
            .strip_suffix(|_| true)
            .expect("no comma at the end of the line"),
    ));

    Route::default()
}
