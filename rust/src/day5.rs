use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::{HashMap, HashSet};


fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();
    lines
}



pub fn part1(filename:&str){
    let lines = read_data(&filename);

    /*
    sort order?
    0 = 47
    1 = 53
    2 = 29
    3 = 13

    issue is that there are multiple after ex 53, so maybe do "what's after" hastmap:list
    53: [29,13]
    when looping the update list, check to see if next number is in hashmap[current] if not then invalid, last one is not key'ed since it has no next to check
     */

    let mut hash_order: HashMap<i32,HashSet<i32>> = HashMap::new();
    let mut reading_order = true;
    let mut center_sum = 0;
    for line in lines{
        if line.trim() == "".to_string() {
            reading_order = false;
        }
        else if reading_order {
            // expecting: key|val

            let line_split: Vec<i32> = line.split('|').map(|val|val.parse::<i32>().unwrap()).collect();
            let key:i32 = line_split[0];
            let val:i32 = line_split[1];
            let mut hash_val = hash_order.entry(key).or_insert(HashSet::new());
            hash_val.insert(val);
        }
        else {
            // expecting (always odd count): val,val,val 

            let line_split: Vec<i32> = line.split(',').map(|val|val.parse::<i32>().unwrap()).collect();
            let mut valid = true;
            for i in 0..(line_split.len()-1){
                let val = line_split[i];
                let next_val = line_split[i+1];
                println!("{val}");
                if !hash_order.get(&val).unwrap_or(&HashSet::new()).contains(&next_val) {
                    valid = false;
                    break
                }
            }
            if valid {
                center_sum += line_split[line_split.len() / 2];
            }
        }
    }
    // println!("{:?}", hash_order);
    println!("Day 5.1 = {center_sum}");
}

pub fn part2(filename:&str){
    let lines = read_data(&filename);

    let mut hash_order: HashMap<i32,HashSet<i32>> = HashMap::new();
    let mut reading_order = true;
    let mut center_sum = 0;
    for line in lines{
        if line.trim() == "".to_string() {
            reading_order = false;
        }
        else if reading_order {
            // expecting: key|val

            let line_split: Vec<i32> = line.split('|').map(|val|val.parse::<i32>().unwrap()).collect();
            let key:i32 = line_split[0];
            let val:i32 = line_split[1];
            let mut hash_val = hash_order.entry(key).or_insert(HashSet::new());
            hash_val.insert(val);
        }
        else {
            // expecting (always odd count): val,val,val 

            let mut line_split: Vec<i32> = line.split(',').map(|val|val.parse::<i32>().unwrap()).collect();
            let mut valid = true;
            for i in 0..(&line_split.len()-1){
                let val = &line_split[i];
                let next_val = &line_split[i+1];
                if !hash_order.get(&val).unwrap_or(&HashSet::new()).contains(&next_val) {
                    valid = false;
                    break
                }
            }
            // assuming rules form a DAG (no cycles) there will always be 1 val with no connections, backtrack from that
            if !valid {
                // create the subset graph from full graph rules
                println!("invalid {:?}", line_split);
                let mut start_val: &i32 = &0;
                let mut subhash_order: HashMap<&i32,HashSet<i32>> = HashMap::new();
                for i in 0..line_split.len(){
                    let val = &line_split[i];
                    // println!("{:?}, {}", hash_order, val);
                    let mut subset = hash_order.get(&val).unwrap();
                    let subset: HashSet<i32> = subset.intersection(&line_split.iter().cloned().collect()).copied().collect();
                    subhash_order.insert(val,subset);
                    if subhash_order.get(val).unwrap().len() == 0 {
                        start_val = val;
                    }
                }

                // look for 0 children node and pop off that key from other sets, loop till there are no more nodes
                let mut next_val = start_val;
                let mut order: Vec<i32> = vec![*next_val];
                while subhash_order.len() != 0 {
                    // let mut subset = subhash_order.get(start_val).unwrap();
                    // println!("{:?}, {}", subhash_order, start_val);
                    subhash_order.remove(start_val);
                    // update child relations now without 0th child
                    for (key, subset) in subhash_order.iter_mut(){
                        subset.remove(start_val);
                        if subset.len() == 0 {
                            next_val = key;
                            order.push(*next_val);
                        }
                    }
                    // println!("{:?}", subhash_order);
                    start_val = next_val;
                    // println!("{}", next_val);
                }
                // note order is technically reversed but we dont care since midpoint is the same fwd/backwards for odd pages
                line_split = order;
                center_sum += line_split[line_split.len() / 2];
            }
            // println!("{:?}", line_split);
        }
    }
    println!("Day 5.2 = {center_sum}");
}
