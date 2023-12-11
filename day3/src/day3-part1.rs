#![allow(dead_code, unused_variables, unused_assignments, unused_imports, unused_labels)]

use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Part{
    partnum : i64,
    pos: HashSet<(i64,i64)>
}


// thank you "Uncle Scientist"!
// https://www.youtube.com/watch?v=QMcfEVVrJlk&t=1415s

fn main() {

    let stream = fs::read_to_string("./input.txt").unwrap();

// // test input
//     let stream = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

    let mut symb:HashSet<(i64,i64)> = HashSet::new();

    let mut parts:Vec<Part> = Vec::new();
    let mut current_num = "".to_string();
    let mut is_digit = false;
    let mut current_poss:HashSet<(i64,i64)> = HashSet::new();

    for (y,line) in stream.lines().enumerate(){
        for (x,char) in line.chars().enumerate(){

            // println!("{} {}",x,y);
            let xx = x as i64;
            let yy = y as i64;

            if char.is_ascii_digit(){
                is_digit = true;
                current_num.push_str(&char.to_string());

                // . . . 
                // . x . 
                // . . .

                let pos:HashSet<(i64,i64)> = HashSet::from([
                    (xx-1 ,yy+1 ), //left 
                    (xx-1 ,yy ),   //left 
                    (xx-1 ,yy-1 ), //left 
                    (xx ,yy+1 ),   // top
                    (xx ,yy-1 ),   // bottom
                    (xx+1 ,yy+1 ), // right
                    (xx+1 ,yy ),   // right
                    (xx+1 ,yy-1 ) // right
                ]); 
                current_poss.extend(pos);

            }else{
                if char!='.'{
                    let pos = (xx,yy);
                    symb.insert(pos);
                }
                if is_digit {
                    is_digit = false;
                    let part = Part{ 
                        partnum: current_num.parse::<i64>().unwrap(),
                        pos: current_poss.clone()
                    };
                    parts.push(part);

                    current_num = "".to_string();
                    current_poss.clear();
                }
            }
        }
    }

    let mut parts_sum:i64 = 0;
    for part in parts{
        if part.pos.intersection(&symb).next().is_some(){
            parts_sum += part.partnum;
        }
    }
    println!("{:?}",parts_sum);

}

