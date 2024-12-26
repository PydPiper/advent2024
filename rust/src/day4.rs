use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::HashMap;


fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();
    lines
}


fn find_word_in_line(word:&str, line:&str) -> i32 {
    let mut count = 0;
    
    let mut next_char = ' ';
    let word_size: i32 = word.len().try_into().unwrap();

    let mut increment: i32 = 1;
    let mut i = 0;
    for char in line.chars(){
        // println!("{char} {next_char}");
        if next_char != ' ' {
            if char == next_char {
                i += increment;
                if i == word_size || i == -1 {
                    // completed word
                    count += 1;
                    next_char = ' ';
                }
                else {
                    next_char = word.chars().nth(i.try_into().unwrap()).unwrap();
                }
            }
            else {
                next_char = ' ';
            }
        }
        // assumption none of the starting chars are inside the word so this is fine, otherwise this cannot be if, needs to be else if
        if char == 'X' {
            increment = 1;
            i = 0;
            i += increment;
            next_char = word.chars().nth(i.try_into().unwrap()).unwrap();
        }
        else if char == 'S' {
            increment = -1;
            i = word_size - 1;
            i += increment;
            next_char = word.chars().nth(i.try_into().unwrap()).unwrap();
        }
    }
    count
}

pub fn part1(filename:&str){
    let lines = read_data(&filename);

    let mut count = 0;
    let word: &str = "XMAS";
    let n_col = lines[0].len();
    let n_row = lines.len();

    // horiz pass
    // println!("horizontal");
    for i in 0..n_row{
        let line = &lines[i];
        count += find_word_in_line(&word, &line);
        // println!("{line} {count}");
    }
    // println!("count = {count}");

    // vertical pass
    // println!("vertical");
    let n_col = n_col;
    for i_col in 0..n_col{
        let mut line = String::new();
        for i_row in 0..n_row{
            line.push(lines[i_row].chars().nth(i_col).unwrap());
        }
        count += find_word_in_line(&word, &line);
    }
    // println!("count = {count}");

    // diag pass left-to-top, however this stops at nth row, so we need to march from bottom right afterwards
    for i_row in 0..n_row{
        let mut line = String::new();
        let mut i_col = 0;
        for i in 0..(i_row+1) {
            line.push(lines[i_row-i].chars().nth(i_col).unwrap());
            i_col += 1;
            if i_col == n_col { break };
        }
        // println!("line = {line}");
        count += find_word_in_line(&word, &line);
    }
    // diag pass bottom-to-right
    println!("diag: bottom-right");
    for i_col in 1..n_col{
        let mut line = String::new();
        // 0,1
        for i_col2 in 0..(n_col-i_col) {
            // i_col=1, i_col2=0
            // i_col=1, i_col2=1
            line.push(lines[(n_row-1) - i_col2].chars().nth(i_col+i_col2).unwrap());
        }
        // println!("line = {line}");
        count += find_word_in_line(&word, &line);
    }
    // println!("count = {count}");

    // diag pass right-to-top
    // println!("diag: right-to-top");
    for i_row in 0..n_row{
        let mut line = String::new();
        let mut i_col: i32 = n_col.try_into().unwrap();
        i_col -= 1;
        for i in 0..(i_row+1) {
            line.push(lines[i_row-i].chars().nth(i_col.try_into().unwrap()).unwrap());
            i_col -= 1;
            if i_col == -1 { break };
        }
        // println!("line = {line}");
        count += find_word_in_line(&word, &line);
    }
    // diag pass bottom-to-left
    // println!("diag: bottom-left");
    for i_col in (0..(n_col-1)).rev(){
        let mut line = String::new();
        // icol=1,0
        for i_col2 in 0..(i_col+1) {
            // icol2=0,1
            // i_col=1, i_col2=0
            // i_col=1, i_col2=1
            line.push(lines[(n_row-1) - i_col2].chars().nth(i_col-i_col2).unwrap());
        }
        // println!("line = {line}");
        count += find_word_in_line(&word, &line);
    }

    println!("Day 4.1 = {count}")
}

pub fn part2(filename:&str){
    let lines = read_data(&filename);

    let n_rows = lines.len();
    let n_cols = lines[0].len();
    // look for "a" then look around it in X to see if it's
    /*
    M.S      S.M      M.M      S.S
    .A.  or  .A.  or  .A.  or  .A.
    M.S      S.M      S.S      M.M
     */
    let mut count: i32 = 0;
    for i_row in 1..(n_rows-1){
        for i_col in 1..(n_cols-1){
            let cur_char = lines[i_row].chars().nth(i_col).unwrap();
            if cur_char == 'A' {
                let topleft_char = lines[i_row-1].chars().nth(i_col-1).unwrap();
                let bottomleft_char = lines[i_row+1].chars().nth(i_col-1).unwrap();
                let topright_char = lines[i_row-1].chars().nth(i_col+1).unwrap();
                let bottomright_char = lines[i_row+1].chars().nth(i_col+1).unwrap();

                if topleft_char == bottomleft_char && topright_char == bottomright_char {
                    if topleft_char == 'M' && topright_char == 'S' {
                        count += 1;
                    }
                    else if topleft_char == 'S' && topright_char == 'M'{
                        count += 1;
                    }
                }
                else if topleft_char == topright_char && bottomleft_char == bottomright_char {
                    if topleft_char == 'M' && bottomleft_char == 'S' {
                        count += 1;
                    }
                    else if topleft_char == 'S' && bottomleft_char == 'M'{
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Day 4.2 = {count}");
}