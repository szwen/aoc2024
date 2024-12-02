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

fn obtain_values_in_line<P>(filename: P)
where P: AsRef<Path>, {
    let mut count:i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let parts= line.split(" ");
            let mut collection: Vec<&str> = parts.collect();
            let mut this = 0;
            let mut next = 0;
            let mut decreasing:bool = true;
            //lets compute the direction first
            //we are going through the vectors in reverse order but that is ok
            this = collection.pop().unwrap().parse::<i32>().unwrap();
            next = collection[collection.len()-1].parse::<i32>().unwrap();
            println!("this {}", this);
            println!("next {}", next);
            if (this < next) {
                decreasing = false;
            }
            // I need to check these two values I got!
            let diff = (this - next).abs();
            println!("diff {}", diff);
            if (diff > 3 || diff <1){
                println!("not the correct diff {}", diff);
            }
            else{
                let mut wrong: bool = false;
                while collection.len() > 1 {
                    this = collection.pop().unwrap().parse::<i32>().unwrap();
                    next = collection[collection.len() - 1].parse::<i32>().unwrap();
                    println!("this {}", this);
                    println!("next {}", next);
                    if (this < next && decreasing) {
                        // wrong progression, lets not continue
                        println!("wrong progression!");
                        wrong = true;
                        break;
                    }
                    if (this > next && !decreasing) {
                        // wrong progression, lets not continue
                        println!("wrong progression!");
                        wrong = true;
                        break;
                    }
                    let diff = (this - next).abs();
                    if (diff > 3 || diff < 1){
                        println!("wrong jump!");
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    //we got to the end of the loop
                    println!("increasing counter");
                    count = count + 1;
                }

            }
        }
    }
    println!("count {}", count);
    // right outcome is 402

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

    obtain_values_in_line("data.txt");

    //println!("Distance: {}",compute_min_distance(vec_left.clone(), vec_right.clone()));
    // expected result: 1879048
    //println!("Similarity: {}",compute_similarity_score(vec_left.clone(), vec_right.clone()));
    // expected result: 21024792

}

