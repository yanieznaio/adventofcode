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

Need 12 digits from 15 available
Algorithm to Find Maximum Joltage
Steps:

Initialize: Start at position 0 of the digit sequence. You need to select exactly 12 digits.
For each selection (repeat 12 times):

Calculate how many digits you still need to select
Calculate the search window: you can look ahead up to position (total_length - digits_still_needed + 1)
Within this search window, find the largest digit
Select the first occurrence of that largest digit
Move your starting position to right after the selected digit


Concatenate all 12 selected digits to form the maximum joltage number.

Why it works:

Search window constraint: Ensures you always have enough digits remaining to complete your selection
Greedy choice: Always picking the largest available digit at each step maximizes the final number
Order preservation: Only selecting digits in their original sequence order (never rearranging)

Example (234234234234278 → 434234234278):

Need 12 from 15 digits (can skip 3)
Position 1: Search first 4 positions "2342" → max is '4' at position 3 → select '4'
Position 2: Search "2342" → max is '4' at position 3 → select '4'
Continue this pattern...
Result: Skip the first "23", then take remaining digits optimally

The key insight: maximize early positions first since they have the most impact on the final number's value

 */
