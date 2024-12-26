use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::HashMap;

use regex::Regex;


fn read_data(filename: &str) -> String{
    let mut file = File::open(filename).expect("unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content);
    // println!("{content}");
    content
}

pub fn part1(filename:&str){
    let content = read_data(&filename);

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&content).map(|cur_match| cur_match.as_str()).collect();
    // println!("{:?}", matches);

    let str_matches = matches.join("");

    let re = Regex::new(r"\d{1,3}").unwrap();
    let matches: Vec<i32> = re.find_iter(&str_matches).map(|cur_match| cur_match.as_str().parse::<i32>().expect("unable to convert str to int")).collect();
    // println!("{:?}", matches);

    let mut total = 0;
    for i in (0..(matches.len()-1)).step_by(2) {
        total += matches[i] * matches[i+1];
    }
    println!("Day 3.1 = {total}")

}

pub fn part2(filename:&str){
    let content = read_data(&filename);

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&content).map(|cur_match| cur_match.as_str()).collect();
    // println!("{:?}", matches);

    let mut str_matches: String = String::new();
    let mut enabled = true;
    for text in matches{
        if text == "do()".to_string(){
            enabled = true;
        }
        else if text == "don't()".to_string() {
            enabled = false;
        }
        else if enabled {
            str_matches += text;
        }
    }

    let re = Regex::new(r"\d{1,3}").unwrap();
    let matches: Vec<i32> = re.find_iter(&str_matches).map(|cur_match| cur_match.as_str().parse::<i32>().expect("unable to convert str to int")).collect();
    // println!("{:?}", matches);

    let mut total = 0;
    for i in (0..(matches.len()-1)).step_by(2) {
        total += matches[i] * matches[i+1];
    }
    println!("Day 3.2 = {total}")
}