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
    let routes = climbs
        .iter()
        .map(|c| parse_climb(c.to_vec()))
        .collect::<Vec<Route>>();
}
