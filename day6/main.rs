


fn do_operations(nums: &vec<i64>, operator: char)
{
        let mut sum = 0;
        let mut i = 0;
        // if the operator is a plus make the sum of all number
        if operator == '+'
        {
            while i < nums.len()
            {
                sum += nums[i];
                i+= 1
            }
        }

        // if the operator is a * make the product of all number
        if operator == '*'
        {
            while i < nums.len()
            {
                sum += nums[i];
                i += 1;
            }
        }
         // then we return the sum of all the number
        sum 
}

fn main()
{
    let mut count = 0;
    let mut vec_2d:Vec<Vec<i64>> = Vec::new();
    let mut op_char:Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./ex.txt")
    {
        for line in lines.map_while(Result::ok)
        {
            let id_string = line.to_string();
            if line.is_empty() {
                continue;
            }
            // if line index is 0
            // splt the line a space and turn turn each part to int
            // the add each one to a new vec and push each vec to a new 
            if is_firstline == 0
            {
                let chunks:Vec<i64> = id_string.split(" ").parse().unwrap().collect();
                vec_2d.push(...chunks);
            }
            // once i Have pushed all the vec in the vec in the first line
            // the next line i can just 
            // split at space
            // and after i just push
            if (checkifoperator(line) == 1)
            {
                 // if it is a character operator then i can just 
                let op_char:Vec<char> = id_string.split(" ").trim().to_string();
                // then i Loop threw all the char and do sum += 
            } 
            else {
            let chunks:Vec<i64> = id_string.split(" ").parse().unwrap().collect()
            let i = 0;
            while i < vec_2d.len()
            {
                vec_2d[i].push(chunks[i]);
                i++;
            }        
            println!("{}", id_string);
            }
        }
    }

    let i = 0;
    while (i < op_char.len())
    {
        count += do_operation(&vec_2d[i], op_char[i]);
    }

    println!("{}", count);

}

/* day 6
//
//the first line will detertmine how many vec in my vec i will need

/* line index is 0
 split at space and collect as vec of i64

the push each num in coresponding vec

after that i keep pushing the num then when i enter the 
end and the fist char corespond to * or + or * or + it will mean 
that it is the last line
so i can start 
i pass the first vec with the char 
and process to the operation

i add the result to a sum

i do that for all the char and all the char


[read the file]

*/
