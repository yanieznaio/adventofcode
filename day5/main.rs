use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn check_if_in_range(ingredient: String, id_range: String) -> i32
{
    //split a - 
    // convert to num
    let mut min;
    let mut max;

    if ingredient >= min && ingredient <= max
    {
        return 1;
    }
    0   
}


fn checkallrange(all_id_range: &Vec<String>>, ingredient: String) -> i32
{
    let mut i = 0;

    while i  < all_id_range.len()
    {
        if check_if_in_range(all_id_range[i], ingredient);
            return 1;
        i++;
    }
    0
}

fn loopthrewvec(all_id_range: &Vec<String>>, all_ingredient: &Vec<String>>) -> i32
{
    let mut i = 0;
    let mut sum = 0;
    while i < all_ingredient.len()
    {
        sum += checkallrange(&all_id_range, all_ingredient[i]);
        i++;
    }
    sum
}

fn main()
{
    let mut count_line_empty = 0;
    let mut all_id_range: Vec<String> = Vec::new();
    let mut all_ingredients: Vec<String> = Vec::new(); 
    if let Ok(lines) = read_lines("./file.txt"){
        for line in lines.map_while(Result::ok){
            let line_string = line.trim().to_string();
            if line.is_empty()
            {
                count_line_empty += 1;
            }
            if count_line_empty == 0
            {
                all_id_range.push(line_string.clone());
            }
            if count_line_empty == 1
            {
                all_ingredients.push(line_string.clone());
            }
        }
    }
    let sum = loopthrewvec(&all_id_range, &all_ingredient);
    printlni!("{}", sum);
    //println!("{:?}", all_id_range);
    //println!("{:?}", all_ingredients);
}



/*
 my thinking for day 5
loop trew file till we are not on a enpty line
and push in vect of str,

tben push the second part in another vect of long type ? I need to find the coresponding type in rust
then once i got these 2 vector
i need to loop thre the second vector

and whitn the second number loop thre the second vecteur
while i < second vector.len 
pass the element of the second vecteur as parameter to a function is_in_range(min , max, num) 
for checking if it is in range 

if yes return true
and increment a counter 
to this for all ingredient id range, 
but we need to increment just one 

one we go threw all te id range
pass to another ingredient
*/
