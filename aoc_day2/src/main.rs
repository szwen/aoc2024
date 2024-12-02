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

fn check_vector_is_valid(mut collection:Vec<&str>) -> bool
{            let mut this = 0;
    let mut next = 0;
    let mut decreasing:bool = true;

    //lets compute the direction first
    //we are going through the vectors in reverse order but that is ok
    this = collection.pop().unwrap().parse::<i32>().unwrap();
    next = collection[collection.len()-1].parse::<i32>().unwrap();
    if (this < next) {
        decreasing = false;
    }
    // I need to check these two values I got!
    let diff = (this - next).abs();
    if (diff > 3 || diff <1){
        false
    }
    else{
        let mut wrong: bool = false;
        while collection.len() > 1 {
            this = collection.pop().unwrap().parse::<i32>().unwrap();
            next = collection[collection.len() - 1].parse::<i32>().unwrap();
            if (this < next && decreasing) {
                // wrong progression, lets not continue
                wrong = true;
                break;
            }
            if (this > next && !decreasing) {
                // wrong progression, lets not continue
                wrong = true;
                break;
            }
            let diff = (this - next).abs();
            if (diff > 3 || diff < 1){
                wrong = true;
                break;
            }
        }
        if !wrong {
            //we got to the end of the loop
            true
        }else{
            false
        }

    }

}

fn verify_values_in_line<P>(filename: P)
where P: AsRef<Path>, {
    let mut count:i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let parts= line.split(" ");
            let mut collection: Vec<&str> = parts.collect();
            let valid = check_vector_is_valid(collection);
            if valid {
                count += 1;
            }


        }
    }
    println!("count {}", count);
    // right outcome is 402

}

fn verify_values_in_line_remove1<P>(filename: P)
where P: AsRef<Path>, {
    let mut count:i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let parts= line.split(" ");
            let mut collection: Vec<&str> = parts.collect();
            let mut valid = check_vector_is_valid(collection.clone());
            if !valid{ //try other combinations
                for n in 0..(collection.len()){
                    let mut collection2 = collection.clone();
                        collection2.remove(n);
                    valid = check_vector_is_valid(collection2);
                    if valid{
                        count += 1;
                        break;
                    }
                }
                //try other combinations

            }
            else {
                count += 1;
            }

        }
    }
    println!("count {}", count);
    // right outcome is 455

}


fn main() {
    println!("Let's try to read some data :D");
    verify_values_in_line("data.txt");
    verify_values_in_line_remove1("data.txt");


}

