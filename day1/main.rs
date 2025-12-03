use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main()
{

   
    let my_dir = "L26";
    let num = &my_dir[1..];
    let distance = &my_dir[..1];
    println!("distance is alphabet {}", distance.chars().isalphabetic());    

    println!("{:?}", num);
    println!("{:?}", distance);
    
    let m_string = num.to_string();
    let num_toint: i32 = m_string.parse::<i32>().unwrap();

    println!("{:?}", num_toint);
    let my_string = "27".to_string();
    let my_int: i32 = my_string.parse::<i32>().unwrap();
    println!("{:?}", my_int);
}


/* 
 !!!!!!!!!

to do
[] open file
[] read file
[] create a struct
[] the first letter l or r in letter
[] second part must be int dans number
[] create an array from 0 to 99  
[] loop trew list 
[] if l go right in the array "it like a clock"
[] if r go left in the array 

exemple 

L30
R48The dial starts by pointing at 50.
L5
R60
L55
L1
L99
R14
L82
Following these rotations would cause the dial to move as follows:

The dial starts by pointing at 50.
The dial is rotated L68 to point at 82.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32.



make an array from 99 to 0 
if l go right 
ig r go left


exemple if the index if at 50

go right so since the array is from 99 to 0 
for  L68
the pointeur would go to O it would make 50 right and then goes backto 0 at the begining
it wil rest 18 moving riight
so 99 98 97 etc to 82
*/
