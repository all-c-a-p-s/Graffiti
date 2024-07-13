#![allow(dead_code)]
#![allow(unused)]

use crate::climb::*;
use crate::json_parse::*;

pub mod climb;
pub mod json_parse;

fn main() {
    //let r = read_route();
    //println!("{}", r);
    let climbs = match get_climbs() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let first_climb = climbs[0].clone();
    let find_test = find(String::from(r#""grade":"#), &first_climb);
    match find_test {
        Some(n) => println!("{}", n),
        None => println!("did not find target string"),
    }
}
