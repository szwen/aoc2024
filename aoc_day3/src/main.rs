use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;



fn read_file(filename: &str) -> String {
    let contents =fs::read_to_string(filename);
    contents.expect("Error reading file")

}

fn locate_dos_donts(s: String){
    let parts = s.split("do");
    let mut sum = 0;
    for part in parts {
        //check whether it is a do or a dont
        if part[0..5].eq("n't()") {
            //check if do-n't()
            println!("don't found!");
        } else if part[0..2].eq("()") {
            //do found
            println!("do found!");
            sum = sum + split_text(part.to_string());
        } else {
            println!("anyway!");
            sum = sum +split_text(part.to_string());
        }
    }
    println!("final sum: {}", sum);
    //answer is
}

fn split_text(s: String)->i64{
    let mut sum = 0;
    let re = Regex::new(r"^[0-9]*$").unwrap();
    let parts= s.split("mul(");
    for part in parts {
        let miniparts = part.split(")");
        let mult_vec: Vec<&str> = miniparts.collect();
        let mult = mult_vec[0].to_string();
        println!("{}", mult);
        let factors = mult.split(",");
        let factors_vec= factors.collect::<Vec<&str>>();
        if factors_vec.len() ==2 { //we are only interested in too numeric operads
            let factor1 = factors_vec[0].to_string();
            let mut factor2;
            if re.is_match(factor1.as_str()){
                println!("factor 1 is valid!");
                factor2 = factors_vec[1].to_string();
                if re.is_match(factor2.as_str()){
                    println!("factor 2 is valid!");
                    sum = sum + factor1.parse::<i64>().unwrap() * factor2.parse::<i64>().unwrap();

                }

            }

        }



    }
    println!("sum is {}", sum);
    sum
    //answer is 173785482

}



fn main() {
    println!("Let's try to read some data :D");
    let f = read_file("data.txt");
    //split_text(f);
    locate_dos_donts(f);


}
