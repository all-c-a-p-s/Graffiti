#![warn(clippy::all)]

use std::fs::File;
use std::io::prelude::*;

use crate::climb::*;

/*
 * Currently I am not actually using any of the code in this file because I ended up just using
 * pandas. It might be useful in the fututre to be able to parse the json in Rust though...
*/

const PATH: &str = "/Users/seba/rs/graffiti/json/2016.json";

const FIRST_CLIMB_LINE_NUMBER: usize = 3;
const LAST_CLIMB_LINE_NUMBER: usize = 5143741;
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
        Ok(s) => Ok(s.lines().map(String::from).collect::<Vec<String>>()),
        Err(e) => Err(e),
    }
}

pub fn next_climb(line_num: usize, lines: &[String]) -> (Option<Vec<String>>, usize) {
    if line_num >= LAST_CLIMB_LINE_NUMBER {
        return (None, line_num);
    }
    let mut bracket_count = 0;
    let mut opened: bool = false;
    let mut new_climb: Vec<String> = Vec::new();
    for (line_num, line) in lines
        .iter()
        .enumerate()
        .take(LAST_CLIMB_LINE_NUMBER + 1)
        .skip(line_num + 1)
    {
        new_climb.push(line.clone());
        for char in 0..line.len() {
            match line.chars().collect::<Vec<char>>()[char] {
                '{' => {
                    bracket_count += 1;
                    opened = true;
                }
                '}' => bracket_count -= 1,
                _ => {}
            }
            if bracket_count == 0 && opened {
                return (Some(new_climb), line_num);
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

pub fn find(target: String, lines: &[String]) -> Option<usize> {
    //find target substring in a vec of lines
    let trimmed = lines
        .iter()
        .map(|l| String::from(l.trim()))
        .collect::<Vec<String>>();
    for (line_num, line) in trimmed.iter().enumerate() {
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
// TODO: name!
pub fn parse_climb(mut lines: Vec<String>) -> Route {
    //basically all climbs are feet follow hands so there isn't even enough data on non-feet follow hands climbs
    //so just assume that the climb is feet follow hands
    let mut route = Route::default();
    let grade_line =
        &lines[find(String::from(r#""grade":"#), &lines).expect("failed to find grade")];
    let grade = String::from(
        grade_line
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .strip_suffix(r#"","#)
            .expect("failed to strip suffix from string")
            .strip_prefix('"')
            .expect("failed to strip prefix from string"),
    );

    let name_line = &lines[find(String::from(r#""name":"#), &lines).expect("failed to find grade")];
    let name = String::from(&name_line.trim()[8..]);

    route.name = name;

    route.grade = Grade::from_font(grade);

    let mut cursor = find(String::from(r#""problemId":"#), &lines);
    let mut count = 0;
    //search for this as this string is unique to the lists of holds
    while cursor.is_some() {
        let unwrapped = cursor.unwrap();
        let hold_position_line = unwrapped + 1; //always on line immediately after

        let hold_position = &lines[hold_position_line]
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()[1][1..=2];

        let is_start_line = hold_position_line + 1;

        let is_start: bool = String::from(
            &lines[is_start_line]
                .split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()[1]
                .strip_suffix(',')
                .unwrap()
                .to_owned(),
        )
        .parse()
        .unwrap();

        let is_end_line = is_start_line + 1;

        let is_end: bool = String::from(
            &lines[is_end_line]
                .split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()[1]
                .to_owned(),
        )
        .parse()
        .unwrap();

        let state = if is_start {
            PositionState::Start
        } else if is_end {
            PositionState::Finish
        } else {
            PositionState::Handhold
        };

        let hold = Hold::from(state, count);
        count += 1;

        let index = name_to_arr_index(hold_position);
        route.holds[index.0][index.1] = hold;

        lines.drain(0..=is_end_line);

        cursor = find(String::from(r#""problemId":"#), &lines);
    }

    route
}
