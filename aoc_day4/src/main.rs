use std::{fs, io};
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

struct Letter {
    value: char,
    coord_x: i32,
    coord_y: i32,
}

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

fn find_keyword_in_grid(grid: &Vec<Vec<char>>, keyword: Vec<char>) -> i32 {
    let mut count:i32 = 0;
    let found:Vec<Vec<Letter>> = Vec::new();
    //aniria buscant un match d'una lletra i provant si cap amunt, avall o reverse la trobo.
    // A la que trobo una paraula intentaria substituir aquells caracters per estrellitas per no contarla dos cops.
    // No puc fer aixo, el que m'haig de guardar son les coordenades. M'he creat una estructura letter per això
    //no tinc clar si haig d'utilitzar un vector o un array i llegir primer la dimensio de la tragedia

    for row in grid {
        for inspected_char in row {
            if let pos = keyword.iter().position(|&c| c == inspected_char.to_owned()).unwrap() {
                println!("found keyword at {:?}", pos);
            }

        }
    }
    count

}

fn main() {
    println!("Let's try to read some data :D");
    let keyword = vec!['X', 'M', 'A', 'S'];
    let grid = convert_to_char_grid("data.txt");
    find_keyword_in_grid(&grid, keyword);


}
