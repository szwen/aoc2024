use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//boilerplate unused code
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

fn read_columns<P>(filename: P) -> (Vec<i32>, Vec<i32>)
where P: AsRef<Path>, {
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();
    // File must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let parts = line.split("   ");
            let collection: Vec<&str> = parts.collect();
            vec_left.push(collection[0].to_owned().parse().unwrap());
            vec_right.push(collection[1].to_owned().parse().unwrap());
            //println!("{}", line);
        }
    }
    (vec_left, vec_right)
}

fn compute_min_distance(mut vec_left: Vec<i32>, mut vec_right: Vec<i32>) -> i32 {
    vec_left.sort();
    vec_right.sort();
    //compute distance vector
    let mut vec_d: Vec<i32> = Vec::new();
    while vec_left.len() != 0 {//we are blatantly assuming both vectors have the same length
        let a = vec_left.pop().unwrap();
        let b = vec_right.pop().unwrap();
        vec_d.push((a - b).abs().try_into().unwrap());
    }
    //add distances
    let sum: i32 = vec_d.iter().sum();
    //return
    sum
    //for the purpose of the test, sum result is: 1879048
}

fn compute_similarity_score(mut vec_left: Vec<i32>, mut vec_right: Vec<i32>) -> i32 {
    //vec_left.sort();
    //vec_right.sort();
    //compute similarity vector
    let mut vec_s: Vec<i32> = Vec::new();
    while vec_left.len() != 0 {
        let a = vec_left.pop().unwrap();
        let b: i32 = vec_right.iter().filter(|x| x.to_owned().eq(&a)).count().try_into().unwrap();
        println!("count {}", b);
        vec_s.push((a*b).try_into().unwrap());
    }
    //add similarities
    let sum: i32 = vec_s.iter().sum();
    //return
    sum
}


fn main() {
    println!("Let's try to read some data :D");

    let (vec_left, vec_right) = read_columns("data.txt");
    println!("Distance: {}",compute_min_distance(vec_left.clone(), vec_right.clone()));
    // expected result: 1879048
    println!("Similarity: {}",compute_similarity_score(vec_left.clone(), vec_right.clone()));
    // expected result: 21024792

}
