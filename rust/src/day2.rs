use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_data(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();

    let mut vec_collector: Vec<Vec<i32>> = vec![];
    for line in lines.into_iter() {
        let line_vec:Vec<i32> = line.split_whitespace().map(|val| val.parse::<i32>().expect("unable to convert str to int")).collect();
        vec_collector.push(line_vec);
    }
    vec_collector
}


pub fn part1(filename:&str){
    // kind of like LIS with additional 1-3 constrain

    // loop all col at the same time starting for index 1, to determine each col inc/dec from 0-1
    // keep track of prev val and see if it meets 1-3 req
    let vec_rows = read_data(&filename);

    let mut valid_count = 0;
    for vec_row in vec_rows {
        let signum = (vec_row[1] - vec_row[0]).signum();
        // println!("{:?}, {}", vec_row, signum);
        let mut is_valid = true;
        for i in 1..vec_row.len() {
            let delta = vec_row[i] - vec_row[i-1];
            if !(0 < delta.abs() && delta.abs() <= 3 && signum == delta.signum()) {
                is_valid = false;
                break;
            }
        };
        if is_valid {
            valid_count += 1;
        }
    }
    println!("Day 2.1: {valid_count}")
}



pub fn part2(filename: &str){
    // kind of like LIS with additional 1-3 constrain

    // loop all col at the same time starting for index 1, to determine each col inc/dec from 0-1
    // keep track of prev val and see if it meets 1-3 req
    let vec_rows = read_data(&filename);

    let mut valid_count = 0;
    for vec_row in vec_rows {

        let vec_row_size = vec_row.len();
        let mut count_inc: Vec<i32> = vec![1; vec_row_size];
        let mut count_dec: Vec<i32> = vec![1; vec_row_size];

        // println!("{:?}", vec_row);
        for i in 1..vec_row_size {
            for j in 0..(i) {
                // println!{"{}, {}", vec_row[i], vec_row[j]};
                let delta = vec_row[i] - vec_row[j];
                if (0 < delta.abs() && delta.abs() <= 3 && delta.signum() > 0 && count_inc[i] < (1+count_inc[j])){
                    count_inc[i] = 1 + count_inc[j];
                }
                if (0 < delta.abs() && delta.abs() <= 3 && delta.signum() < 0 && count_dec[i] < (1+count_dec[j])){
                    count_dec[i] = 1 + count_dec[j];
                }
            }
        }
        let compare_size  = &i32::try_from(vec_row_size-1).unwrap();
        let max_inc_count = count_inc.iter().max().unwrap();
        let max_dec_count = count_dec.iter().max().unwrap();
        // println!("LIS = {max_inc_count}, LDS = {max_dec_count}");
        if max_inc_count >= compare_size  || max_dec_count >= compare_size {
            valid_count += 1;
        }
    }

    println!("Day 2.2: {valid_count}")
}