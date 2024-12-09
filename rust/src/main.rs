use std::fs::File;
use std::io::{self, BufRead};


fn read_data(filename: &str) -> io::Result<Vec<String>> {
    // the "?" propagates file open error up out of this function. This is why the function is a Result<...> 
    // "Result" is OK(actual return of your function) or Err(error code)
    // you have to handle errors of this function once it's called thats (for example with a try/except, thats why it has a .expect(...))
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn main() {
    let filename = "src/day1.txt";

    // since read_data can raise an error it has it be handled with .expect to raise the error
    // note this is what converts lines from Result<Vec<String>> to <Vec<String>> because if error isnt raise it can only return the correct type of Vec<String>
    let lines = read_data(filename).expect("Error");
    println!("{:?}",lines); // note this need :? because it doesnt know how to print a vector as str
    for (i,line) in lines.into_iter().enumerate() {
        println!("Line {}, {}", i+1, line);
    }
}


