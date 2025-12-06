use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn isunvalidid(id: i64) -> bool
{
    // Check if the number has repeating patterns
    let id_string = id.to_string();
    let len = id_string.len();
    
    //PART 1 CODE
    // Check if first half equals second half (like 1212, 123123)
    /*if len % 2 == 0 {
        let mid = len / 2;
        let first_half = &id_string[..mid];
        let second_half = &id_string[mid..];
        if first_half == second_half {
            return true;
        }
    }
    */

    //part 2 code
    for pattern_len in 1..=(len / 2) {
        if len % pattern_len == 0 {
            let pattern = &id_string[..pattern_len];
            let repetitions = len / pattern_len;

            // build what the full string would be if the pattern repeated
            let repeated = pattern.repeat(repetitions);
            if repeated == id_string && repetitions >= 2 {
                return true;
            }

        }
    }

    false
}

fn checkids(start: i64, end: i64) -> i64
{
    // this function take start and end loop threw id 
    // check if valid and return sum
    let mut sum = 0;
    let mut i = start;
    
    while i <= end{
        if isunvalidid(i){
            //println!("{}", i);
            sum += i;
        }
        i += 1;
    }
    return sum;
}

fn main(){
    // create a vec of string that will contain all my value
    let mut all_ids: Vec<String> = Vec::new();
    let mut sum = 0;
    if let Ok(lines) = read_lines("./file.txt") {
        for line in lines.map_while(Result::ok){

           // split by commas and push into vectoer
           for id in line.split(',') {
                let id_string = id.trim().to_string();
                if id_string.is_empty() {
                    continue;
                }
                let chunks:Vec<&str> = id_string.split("-").collect();
                let start: i64 = chunks[0].parse().unwrap();
                let end: i64 = chunks[1].parse().unwrap();
                sum += checkids(start, end);
           }
        }
    }
    println!("{}", sum);
}




/* to do first , i need to 
    [] split between commas, 
    [] split between - 
    [] store first in start
    [] store second in end
    [] make loop
    [] i = start
    [] while i <= end
    [] check if the curr is valid with function isvalid
    [] is valid contain these condition
    [] if number is made only of sequence of digits repeated twice
    [] repeated twice.
    [] so 55 twice
    [] 6464  64 twice, and 123123 123 tiwce
    [] after i sum of all he id 
    [] aparently there is a regec for this pattern
    [] ^(\d+)\1
    explanation of this pattern : debut de la chaine
    (\d+) capture un ou plusieurs chiffres dans le groupe 1
    \1: reference arriere (backreference) qui doit correspondre exactement a ce qui
    a ete capture dans le groupe 1
    $ fin de la chaine
    exemple 
    Texte à analyser : 123123
    Regex : ^(\d+)\1$

    Étape 1 : ^ 
    → On est au début ✓

    Étape 2 : (\d+)
    → Cherche des chiffres, trouve "123"
    → Mémorise dans groupe 1 : "123"

    Étape 3 : \1
     → Doit correspondre au contenu du groupe 1, donc "123"
    → Compare avec les caractères suivants : "123"
    → Ça correspond ! ✓

    Étape 4 : $
    → On est à la fin ✓

    Résultat : MATCH !

    Texte à analyser : 123123
    Regex : ^(\d+)\1$

    Étape 1 : ^ 
    → On est au début ✓

    Étape 2 : (\d+)
    → Cherche des chiffres, trouve "123"
    → Mémorise dans groupe 1 : "123"

    Étape 3 : \1
    → Doit correspondre au contenu du groupe 1, donc "123"
    → Compare avec les caractères suivants : "123"
    → Ça correspond ! ✓

    Étape 4 : $
    → On est à la fin ✓

    Résultat : MATCH !
    */
