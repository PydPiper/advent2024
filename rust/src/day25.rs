use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::{HashMap, HashSet};

/*
lock with 1st row all #, ignoring the 1st/last row you get lock height columns = 0,5,3,4,3
#####
.####
.####
.####
.#.#.
.#...
.....

key with 1st row all ., ignoring the 1st/last row you get key height columns = 5,0,2,1,3
.....
#....
#....
#...#
#.#.#
#.###
#####

a match is found when lock_col + lock_key = 5 for all cols, for the above example all but last col = 5 (last col = 6)

*/

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();
    lines
}

fn count_from_line(line: &String, mut col_count: [i32;5]) -> [i32; 5]{
    for (i_char, char) in line.chars().enumerate() {
        if char == '#' {
            col_count[i_char] += 1;
        }
    }
    col_count
}

fn keylock_from_lines(lines: Vec<String>) -> [Vec<[i32;5]>; 2] {
    // let locks: Vec<[i32;5]> = Vec::new();
    // let keys: Vec<[i32;5]> = Vec::new();
    let mut rv: [Vec<[i32;5]>; 2] = [Vec::new(), Vec::new()];
    let mut i_line = 0;
    while i_line < lines.len(){
        let line = &lines[i_line];
        // 0 for lock, 1 for key, 2 for none
        let mut i_type = 2;
        // check if 0:lock or 1:key
        if line.chars().nth(0).unwrap() == '#' {
            i_type = 0;
        }
        else if line.chars().nth(0).unwrap() == '.' {
            i_type = 1;
        }

        // parse into lock/key
        if i_type < 2 {
            let mut col_count: [i32; 5] = [0; 5];

            i_line += 1;
            let mut line = &lines[i_line];
            // assumption: lock/key data is 5x5
            for i in 1..6 {
                col_count = count_from_line(line, col_count);
                i_line += 1;
                // if i_line == lines.len(){ break };
                line = &lines[i_line];
                // println!("{i_line},{line}");
            }
            // skip over empty line
            i_line += 1;

            // while (line.trim().len() != 0) {
            //     col_count = count_from_line(line, col_count);
            //     i_line += 1;
            //     if i_line == lines.len(){ break };
            //     line = &lines[i_line];
            // }

            rv[i_type].push(col_count);
        }

        i_line += 1;

    }

    rv
}

pub fn part1(filename: &str) {
    let lines = read_data(&filename);
    let [locks, keys] = keylock_from_lines(lines);
    // println!("keys\n{:?}", keys);
    // println!("locks\n{:?}", locks);

    let mut valid_count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut valid = true;
            for (lock_val, key_val) in lock.iter().zip(key){
                let allow_val: i32 = 5 - *lock_val;
                if key_val > &allow_val {
                    valid = false;
                }
            }
            println!("lock = {:?}, key = {:?}, valid = {}", lock, key, valid);
            if valid {
                valid_count += 1;
            }
        }
    }
    println!("Part 25.1 = {valid_count}");
}

pub fn part2(filename: &str) {
    
}