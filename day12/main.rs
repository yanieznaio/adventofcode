use std::fs::File;
use std::path::Path;
use std::io::{Self, BufRead};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main()
{
    if let Ok(lines) = read_lines("./file.txt")
    {
        for line in lines.map_while(Result::ok)
        {
            let id_string = line.to_string();
            if line.is_empty() {
                continue;
            }
        }
    }

}
