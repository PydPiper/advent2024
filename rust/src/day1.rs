use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_data(filename: &str) -> [Vec<i32>;2] {
    // the "?" propagates file open error up out of this function. This is why the function is a Result<...> 
    // "Result" is OK(actual return of your function) or Err(error code)
    // you have to handle errors of this function once it's called thats (for example with a try/except, thats why it has a .expect(...))
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    // NOTE: .collect transforms an iterator into a vec/hashmap/string etc.
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();

    // NOTE: the variable assigned from a .collect() MUST be typed, because Rust cannot figure out what .collect() type you want: vec/hashmap/string?
    // however Rust can figure out what the underlying values of the vec is which is why you could say Vec<String> or Vec<_> and let it figure out its strings
    // let numbers = vec![1, 2, 3];
    // let strNumbers: Vec<String> = numbers.iter().map(|x| format!("{x}")).collect();
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];
    for line in lines.into_iter() {
        let split_line:Vec<_> = line.split_whitespace().collect();
        // print!("{line}");
        // print!("{split_line:?}");
        let val1: i32 = split_line[0].parse().expect("error str to int conversion");
        let val2: i32 = split_line[1].parse().expect("error str to int conversion");
        // println!(" {val1}")
        vec1.push(val1);
        vec2.push(val2);
    }
    [vec1, vec2]
}


pub fn part1(filename: &str){
   
    let [mut vec1, mut vec2] = read_data(&filename);

    vec1.sort();
    vec2.sort();

    let mut total: i32 = 0;
    for (val1, val2) in vec1.iter().zip(vec2.iter()){
        // print!("{val1} {val2}\n");
        total += (val1 - val2).abs();
    }
    println!("Day 1.1 = {total}");
}

pub fn part2(filename: &str){
    let [mut vec1, mut vec2] = read_data(&filename);
    // loop over vec2 and tally up unique count (hashmap)
    // loop over vec1 and perform similarity calc based on hashmap vec2 data

    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    for key in vec2{
        // "entry" is an inplace modification to a specific key's keyue. 
        // Here i am using it as on-err inplace fill it with 0 (0 because we increment the count afterwards +1)
        let count = hashmap.entry(key).or_insert(0);
        *count += 1;
    }

    let mut total: i32 = 0;
    for key in vec1{
        let count = hashmap.get(&key).unwrap_or(&0);
        total += key * count;
    }
    println!("Day 1.2 = {total}")
}
