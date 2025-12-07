use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// this function return the number of paper roll on the 8 adjacent pos
// of the current pos
fn check_adjacent(vec: &Vec<Vec<char>>, i : usize, j : usize) -> i32 {
    let mut count = 0;

    /*
    [x]gauche grid[currrow][currcol - 1];                                          [x] haut grid[currrow -1][currcol];                                      
    bat grid[currrow + 2][currcol];                                            
    haut gauche [currow - 1][currcol -1]                                       
    bas gauche [currow + 1][curcol -1]                                         
    bas droite [currow + 1][curcol +1 ]  
    */
    let rows = vec.len() - 1;
    let cols = vec[0].len() - 1; // assumin non empty  and unifirom 
    
    if j > 0 {
        // gauche
        if vec[i][j - 1] == '@' {
            count += 1;
        }
        if i > 0 {
            // bas gauche
            if vec[i - 1][j - 1] == '@' {
                count += 1;
            }
        }
        // check if we can icrement i 
        if i < rows { 
           //bas gauche
           if vec[i + 1][j - 1] == '@' {
               count += 1;
            }
        }
    }
    if j < cols {
        // droite
        if vec[i][j + 1] == '@' {
            count += 1;
        }
    }
    if i > 0 {
        //haut
        if vec[i - 1][j] == '@' {
            count += 1;
        }
        if j < cols {
            // haut droite
            if vec[i - 1][j + 1] == '@' {
                count += 1;
            }
        }
  
    }
    if i < rows {
        // bas
        if vec[i + 1][j] == '@' {
             count += 1;
        }
        // bas droitre
        if j < cols {
            if vec[i + 1][j + 1] == '@' {
                count += 1;
            }
        }
    }
    
    count
}

fn remove_roll(vec : &mut Vec<Vec<char>>, i : usize , y : usize){
    vec[i][y] = 'x'
}

fn loop_vec(vec: &mut Vec<Vec<char>>) -> i32
{
    let mut count = 0;
    // check all the ajacent pos
    // check all the adjactent row
    for (i, row) in vec.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '@'{
                println!("Found '{}' at i = {}, j = {}", c, i, j);
                if check_adjacent(vec, i, j) < 4 {
                    count += 1;
                    //remove roll if < 4
                    remove_roll(vec, i, j);
                }
            }
        }
    } 
    // return count 
    count
}

fn main(){
    let mut count = 0;
    let rm_possible = true;
    let mut vec_2d:Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./file.txt"){
        for line in lines.map_while(Result::ok){
            let id_string = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            let chars: Vec<char> = id_string.chars().collect();
            vec_2d.push(chars);
            println!("{}", id_string);
        }
    }
    println!("{:?}", vec_2d);
    //loop trew vecteur and remove possible roll till is possibi    // to remove
    while rm_possible
    {
        let currcount = loop_vec(&mut vec_2d);
        if currcount == 0 {
            let _rm_possible = false;
            continue;
        }
        else {
            count += currcount;
        }
    }
    println!{"{}", count};
}


/*
 only access paper if there are less than four roll in the eight adjacent positions
if char = @
let sum = 0;
check 8 adjacent
if the position is @ count +

my deduction of the 8 adjactent pos if dynamic array
 our pos => grid[currrow][curcoll]
    droite grid[currrow][currcol -1];
    gauche grid[currrow][currcol - 1];
    haut grid[currrow -1][currcol];
    bat grid[currrow + 2][currcol];
    haut gauche [currow - 1][currcol -1]
    haut droite [currow - 1][currcol + 1]
    bas gauche [currow + 1][curcol -1] 
    bas droite [currow + 1][curcol +1 ]

what I think i need to do
read file and push into vector or dynanic array if exist in rust
then loop from the array, if the element is a roll paper "@"
then check eight adjacent position like above.
if the element of the adjectent pos is equal to roll paper count ++;
add the end of this loop we check if the sum is < 4
if so count_roll_to_aceed ++;

if the row is 0 there will be no upper 
if the col is 0 there will be no left 
if the col is length - 1 there will be no right
if the row is length  - 1 there will be no dow(bas)
*/





