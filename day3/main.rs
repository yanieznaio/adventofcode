use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}


fn findmax(banks: &Vec<i32>, start: usize, end: usize) -> usize {
    // what to take as parameter ? vect or array
    let mut i = start;
    let mut curr = 0;
    let mut bigindex = 0;

    while i < end
    {
        let digit = banks[i];
        if digit > curr 
        {
             curr = banks[i];
             bigindex = i;
        }
        i += 1;
    }
    bigindex
    //index
}

fn main()
{
    let mut sum = 0;
    if let Ok(lines) = read_lines("./file.txt"){
        for line in lines.map_while(Result::ok){
            let banks = line.trim().to_string();
            let v: Vec<i32> = banks.chars().filter_map(|ch| ch.to_digit(10)).map(|d| d as i32).collect();
            println!("{:?}", v);
           
            if v.len() < 2{
                continue;
            }
            let firstbig = findmax(&v, 0, v.len());
            if firstbig == v.len() - 1 {
                let secondbig =  findmax(&v, 0, firstbig);
                let voltage_str = format!("{}{}", v[secondbig], v[firstbig]);
                let voltage : i32 = voltage_str.parse().unwrap(); 
                sum += voltage;
                println!("{}", voltage);
            } else { 
                let secondbig = findmax(&v, firstbig + 1, v.len());
                let voltage_str = format!("{}{}", v[firstbig], v[secondbig]);
                let voltage : i32 = voltage_str.parse().unwrap();
                sum += voltage;
                println!("{}", voltage);
            };
            // now i need to convert voltage to num; 

        
        }
    }
    println!("{}", sum);
}


/*
what i need to do for day 3 is find the most bigger number in the first part 
and fin the bigger number in the other part of the line
so 
check the first
find the bigger number with findmax()
save the index;
then find max (arr, start, end) with start index, end len
or if findmax = len - 1 , findmax(arr, start 0, end findmap index
return the index save
addition the number in 
from start index of bigger(not compris) and end,
then sum += those the number
return the sum
 */
