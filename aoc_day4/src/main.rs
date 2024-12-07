use std::{fs, io};
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn read_file(filename: &str) -> String {
    let contents =fs::read_to_string(filename);
    contents.expect("Error reading file")

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}

fn convert_to_char_grid<P>(filename: P) -> Vec<Vec<char>>
where P: AsRef<Path>, {
    let mut grid: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // put each part of the line in a vector
        for line in lines.flatten(){
            let row: Vec<char> = line.chars().collect();
            grid.push(row);
        }
    }
    grid

}

fn find_keyword_in_grid(grid: &Vec<Vec<char>>, keyword: [char;4]) -> i32 {
    let mut count:i32 = 0;
    //aniria buscant un match d'una lletra i provant si cap amunt, avall o reverse la trobo.
    // A la que trobo una paraula intentaria substituir aquells caracters per estrellitas per no contarla dos cops.
    //no tinc clar si haig d'utilitzar un vector o un array i llegir primer la dimensio de la tragedia

}

fn main() {
    println!("Let's try to read some data :D");
    let keyword = ['X', 'M', 'A', 'S'];
    let grid = convert_to_char_grid("data.txt");


}
