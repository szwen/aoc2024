use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{absolute, Path};

fn read_file(filename: &str) -> String {
    let contents =fs::read_to_string(filename);
    contents.expect("Error reading file")

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}

fn use_lines<P>(filename: P)
where P: AsRef<Path>, {
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();
    // File must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let mut parts = line.split("   ");
            let collection: Vec<&str> = parts.collect();
            vec_left.push(collection[0].to_owned().parse().unwrap());
            vec_right.push(collection[1].to_owned().parse().unwrap());
            println!("{}", line);
            for c in collection.iter() {
                println!("part {}", c)
            }
            //for v in vec_left.iter(){
            //    println!("vector left {}", v)
            //}
        }
        vec_left.sort();
        vec_right.sort();
        //compute distance vector
        let mut vec_d: Vec<i32> = Vec::new();
        while vec_left.len() != 0 {
            let a = vec_left.pop().unwrap();
            let b = vec_right.pop().unwrap();
            vec_d.push((a - b).abs().try_into().unwrap());
        }
        let sum: i32 = vec_d.iter().sum();
        println!("Sum: {}",sum)



    }

}


fn main() {
    println!("Hello, world!");
    println!("Let's try to read some data :D ");
    //let data = read_file("data.txt");
    //println!("Data: {:?}", data);
    use_lines("data.txt");
}
