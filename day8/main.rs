use std::fs::File;
use std::path:Path;
use std::io::{self, BufRead};


fn read_lines<P>(filename: P) -> io::Result::<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    
    if let Ok(lines) = read_lines("./ex.txt")
    {
        for line in lines.map_while(Result::ok)
        {
            let it_string = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            let cha

        }


    }


}


// il y  a des box avec x y z
//
la premiere est x 162 y 817 z 812
// cette bxoe devra etre connecter a 
 x 425 y 690 et z 689

je pense qu'il faut calculer 
les 3 numeros - celui a qui on recherge le plus pres
faire une fonction findmin 
si il est plus pres que celui d'avant continuer 

essayon de calculer la diffrence pour voiravec 

mais comment les deplacer que faire une fois qu'on
a trouver le plusproche 

les mettre dans un nouveau vec 


find min de tout les element du tableau
faire un tableau ou vecteur de longueur equivalent contenant
le min de chaque ligne avec leur index

par exemple 




 
57, 618, 57  => 105, 199, 755
906,360,560  => 744, 457, 252
592,479,940  => 113, 338, 128
352,342,300  => 190, 475, 512
466,668,158  => 304, 149, 654
542,29,236
431,825,988
739,650,466
52,470,668 => 110, 347, 144
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344 =>
425,690,689 => 263, 127 , 123 503


// comment arriver a ce resultat
