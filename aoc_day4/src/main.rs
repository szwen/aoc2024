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



fn search_onwards(grid: &Vec<Vec<char>>, row:&Vec<char>, mut found_word: Vec<(char, i32, i32)>, mut keyword: Vec<char>, mut x_index:usize, y_index:usize)-> Vec<(char, i32, i32)> {
    while keyword.len() > 0{
        if(x_index < row.len() &&  keyword[0].eq(&grid[y_index][x_index])) {
            println!("found keyword {} onwards in grid coords {}, {}, value {}", keyword[0], y_index, x_index, grid[y_index][x_index]);
            /*let letter: Letter = Letter {
                value: grid[y_index][x_index],
                coord_x: x_index as i32,
                coord_y: y_index as i32
            };*/
            let letter:(char, i32, i32)=(grid[y_index][x_index],x_index as i32,y_index as i32);
            found_word.push(letter);
            x_index += 1;
            keyword=keyword[1..].to_vec();
        }else{
            println!("word didn't finish! not found keyword {}", keyword[0]);
            break;
        }

    }
    found_word
}

fn search_backwards(grid: &Vec<Vec<char>>, row:&Vec<char>, mut found_word: Vec<(char, i32, i32)>, mut keyword: Vec<char>, mut x_index:usize, y_index:usize)-> Vec<(char, i32, i32)> {
    while keyword.len() > 0{
        if(x_index >= 0 &&  keyword[keyword.len()-1].eq(&grid[y_index][x_index])) {
            println!("found keyword {} backwards in grid coords {}, {}, value {}", keyword[keyword.len()-1], y_index, x_index, grid[y_index][x_index]);
            /*let letter: Letter = Letter {
                value: grid[y_index][x_index],
                coord_x: x_index as i32,
                coord_y: y_index as i32
            };*/
            let letter:(char, i32, i32) = (grid[y_index][x_index], x_index as i32, y_index as i32);
            found_word.push(letter);
            if x_index>0{
                x_index -= 1;
            }else{
                break;
            }

            keyword=keyword[..keyword.len()-1].to_vec();
        }else{
            println!("word didn't finish! not found keyword {}", keyword[keyword.len()-1]);
            break;
        }

    }
    found_word
}



fn find_keyword_in_grid(grid: &Vec<Vec<char>>, keyword: Vec<char>) -> i32 {
    let mut count:i32 = 0;
    let mut x_index:usize = 0;
    let mut y_index:usize = 0;
    let mut found:Vec<Vec<(char, i32, i32)>> = Vec::new();

    for row in grid {
        x_index = 0;
        for inspected_char in row {
            if let mut pos = keyword.iter().position(|&c| c == inspected_char.to_owned()).unwrap() {
                println!("found keyword {:?} in grid coords {}, {}", keyword[pos], x_index, y_index);
                let mut found_word:Vec<(char, i32, i32)> = Vec::new();
                let mut letter:(char, i32, i32)=(inspected_char.to_owned(), x_index as i32, y_index as i32);
                /*let mut found_word:Vec<Letter> = Vec::new();
                let letter:Letter = Letter {
                    value: inspected_char.to_owned(),
                    coord_x: x_index as i32,
                    coord_y: y_index as i32
                };*/
                found_word.push(letter);//remember that his word will have the letters in any order!
                //char found, now check next or previous in all directions
               if pos < keyword.len()-1{
                   let mut new_pos = pos+1;
                   let mut new_keyword =keyword[new_pos ..].to_vec();
                   let mut new_x_index = x_index+1;
                   let mut onwards_found_word = found_word;


                   //ONWARDS AND THEN BACKWARDS
                   onwards_found_word = search_onwards(grid, row, onwards_found_word, new_keyword.clone(), new_x_index, y_index);

                       if pos > 0 {
                           new_pos = pos - 1;
                           new_keyword = keyword[..pos].to_vec();
                           if x_index > 0 {
                               new_x_index = x_index - 1;
                               onwards_found_word = search_backwards(grid, row, onwards_found_word, new_keyword, new_x_index, y_index);
                           }
                       }
                   if onwards_found_word.len() == keyword.len() {
                       //all letters found!
                       onwards_found_word.sort();


                       if found.contains(&onwards_found_word){
                           println!("word was already there! {:?}", onwards_found_word);
                       }else{
                           println!("word was not already there! {:?}", onwards_found_word);
                        onwards_found_word.sort();
                       found.push(onwards_found_word);

                       }
                   } /*else {
                       //try other combinations if onwards didn't work
                       //try reverse
                       new_pos = pos+1;
                       new_keyword =keyword[new_pos ..].to_vec();
                       new_x_index = x_index-1;
                       let mut backwards_found_word = found_word;
                   }*/



                    /*if (x_index-1>=0 && keyword_char == grid[y_index][x_index-1]) {
                        println!("found keyword {} backwards in grid coords {}, {}", keyword_char, x_index-1, y_index);
                    }else if (y_index-1>=0 && keyword_char == grid[y_index-1][x_index]){
                        println!("found keyword {} upwards in grid coords {}, {}", keyword_char, x_index, y_index-1);
                    }else if (y_index+1 <grid.len() && keyword_char == grid[y_index-1][x_index]){
                        println!("found keyword {} downwards in grid coords {}, {}", keyword_char, x_index, y_index+1);
                    }else{
                        println!("not found ");
                        break;

                    }*/

                }
            }

            println!("x_index: {}, y_index: {}", x_index, y_index);
            println!("grid: {}", grid[y_index][x_index]);
            x_index +=1;
        }
        y_index += 1;
    }
    println!("found words! {}", found.len() as i32);
    found.len() as i32

}

fn main() {
    println!("Let's try to read some data :D");
    let keyword = vec!['X', 'M', 'A', 'S'];
    let grid = convert_to_char_grid("test_data.txt");
    find_keyword_in_grid(&grid, keyword);


}
