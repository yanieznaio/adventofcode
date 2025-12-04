use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::array::from_fn;
use std::fs::read_to_string;
use std::io::{self, BufRead};

// (aparently a working but naive approach that work for beginer for reading line from a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main()
{ 
    // this will create an array from 0 to 99
    // when i asked the question we propose me array  if size know at compile time
    //if i truly need fixed length ,
    //if i wanted  dynamic or flexible size and common operation like push/pop  i could have use a vector(Vec)
    //in this case i know the size since the cadran is from zero to 99 plus i dont need to modify
    //element just to watch with an index.
    //so then it will be the array;
    let arr: [u32; 100] = from_fn(|i| i as u32);
    let mut initial = 50;
    let mut countzero = 0;
    println!("{:?}", arr); 
    
    // now its time to open file;
    if let Ok(lines) = read_lines("./file2.txt") {
        // Consume the iterator , returns an (optional) String
        for line in lines.map_while(Result::ok){
            let instruction = line.clone();
            let num = &instruction[1..];
            let letter = &instruction[..1];
            /*if let Some(c) = letter.chars().next(){
                println!("my letter: {}", c);
            } else 
            {
                println!("String is empty");
            }*/
            let time_string = num.to_string();
            let mut time: i32 = time_string.parse::<i32>().unwrap();
            println!("instruction {}{} ",letter, time);

            if letter.chars().next() == Some('R'){
                //if its equal to right make sum; 
                // count each click 
                let pass_by_zero = time / 100; 
                countzero += pass_by_zero;

                time = time % 100;
                initial = (initial + pass_by_zero
                println!("Go to point :{}", initial);
            }

            else if letter.chars().next() == Some('L')
            {
                
                println!("initial {} - time {} = {}", initial, time, initial - time);
                for _ in 0..time {
                    initial = initial - 1;
                    if initial < 0 {
                        initial = 99;
                    }
                    if initial == 0{
                        countzero += 1;
                        println!(" -> clicked thgrough 0!")
                    }
                    
                }
                println!("Go to point:{}", initial);
            }
      

        println!("{}", line);
        // here push in array if L put minus before;
        //find the algo for this
        // if at the instruct current + instruction its equal a 0
        // increment the counteer of zero.
        }
        println!("final count zero {}", countzero - 1);
    }
    // return the counterofzero
    // extract line;
    // asigned to temp variable
    // and push to a list each time
}


/* 
 !!!!!!!!!

to do
[] open file
[] read file
[] create a struct
[] the first letter l or r in letter
[] second part must be int dans number
[x] create an array from 0 to 99  
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
