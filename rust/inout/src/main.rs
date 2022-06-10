use std::fs::{File, read_to_string};
use std::io::{Write, BufReader, BufRead, Error, Read};

fn main() {
    println!("Input/Output test!\n");

    //Read a file
    let contents = read_to_string("/home/tempura/meow").expect("Something went wrong reading this file");
    println!("File:\n{}", contents);

    //Input
    let input = File::open(path);
    let buffered = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line);
    }

}
