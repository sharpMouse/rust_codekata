#![cfg(test)]
use crate::common::DATA_DIR;

use itertools::Itertools;
use std::path::Path;


fn parse_weather(numbers_line: &str) -> (u8, u8, u8) {
    numbers_line
        .split(&[' ', '*'])
        .filter(|s| !s.is_empty())
        .take(3)
        .map(|x| x.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn parse_football(numbers_line: &str) -> (String, u8, u8) {
    let mut iter = numbers_line.split_whitespace();
    let name = iter.nth(1).unwrap();
    let for_ = iter.nth(4).unwrap().parse().unwrap();
    let against = iter.nth(1).unwrap().parse().unwrap();
    (name.to_string(), for_, against)
}

fn get_smallest_spread<Type>(
    filename: &Path,
    headlines: usize,
    filter_str: &str, 
    parser: &dyn Fn(&str) -> (Type, u8, u8)) 
    -> Type {
    let content = std::fs::read_to_string(filename).unwrap();
    content
        .lines()
        .skip(headlines)
        .filter(|str| !str.contains(filter_str))
        .map(|str| parser(str))
        .min_by_key(|x| x.1.abs_diff(x.2))
        .unwrap()
        .0
}

fn get_smallest_spread_weather(filename: &Path) -> u8 {
    get_smallest_spread(filename, 2, "mo", &parse_weather)
}

fn get_smallest_spread_football(filename: &Path) -> String {
    get_smallest_spread(filename, 1, "--", &parse_football)
}

#[test]
fn test_weather_spread() {
    let filename = DATA_DIR.join("weather.dat");
    assert_eq!(14, get_smallest_spread_weather(&filename));
}

#[test]
fn test_football_spread() {
    
    let filename = DATA_DIR.join("football.dat");
    assert_eq!("Aston_Villa", get_smallest_spread_football(&filename));
}
