use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::{HashMap, HashSet};

/*
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02

- key map the 1st set as key:val
- loop 2nd set and map it to key:val based on bool logic
- for solution loop all z00-z## assembling binary value ### that needs to be decimal conversion
*/

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();
    lines
}

pub fn part1(filename: &str) {
    let mut lookup: HashMap<&str,i32> = HashMap::new();
    let lines = read_data(&filename);

    let mut read_wire_data = true;
    let mut logic_lines: Vec<&String> = Vec::new();
    for line in lines.iter(){
        if line.trim() == "" {
            read_wire_data = false;
        }
        else if read_wire_data {
            // wire data
            let line_split: Vec<&str> = line.split(":").collect();
            let key = line_split[0];
            let val: i32 = line_split[1].trim().parse().unwrap();
            lookup.insert(key, val);
        }
        else {
            // logic data
            logic_lines.push(line);
        }
    }

    let mut i_line = 0;
    while logic_lines.len() != 0 {
        let line = logic_lines[i_line];
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let key1 = line_split[0];
        let logic = line_split[1];
        let key2 = line_split[2];
        let key3 = line_split[4];

        if lookup.contains_key(key1) && lookup.contains_key(key2) {
            // process and pop, but not i_line inc
            let val1 = lookup.get(key1).unwrap();
            let val2 = lookup.get(key2).unwrap();
            let mut val = 0;
            let val = match logic {
                "AND" => val1 & val2,
                "OR" => val1 | val2,
                "XOR" => val1 ^ val2,
                _ => panic!("FREAK OUT! logic unmatched")
            };
            lookup.insert(key3, val);
            logic_lines.remove(i_line);
        }
        else {
            // skip on this pass
            i_line += 1;
        }
        if i_line == logic_lines.len() {
            i_line = 0;
        }
    }

    // println!("{:?}", lookup)

    let mut decimal: i64 = 0;
    for key in lookup.keys() {
        if key.chars().nth(0).unwrap() == 'z' {
            let nth_bit: u32 = key[1..].parse().unwrap();
            let val: i64 = *lookup.get(key).unwrap() as i64;
            println!("{key} {val}");
            // val * 2 ^ (n-th bit) all added up
            decimal += val * 2_i64.pow(nth_bit);
        }
    }
    println!("Part 24.1 = {decimal}")

}

pub fn part2(filename: &str) {
    
}
