#![allow(dead_code, unused_variables, unused_assignments, unused_imports, unused_labels)]

use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point{
    x:i32,
    y:i32
}

#[derive(Debug,Clone)]
struct Part{
    num:u32,
    valid: bool,
    p: Vec<Point>
}

fn main() {

    let stream = fs::read_to_string("./input.txt").unwrap();

    let stream = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
-.592...22
2.....755.
+..$.*....
.664.598..";

// that number 2 doesn't see the - and + above and below!!!

    // char_points is a 2D_array (Vec<Vec<char>>) version of the input string
    // parts store each characters and its x,y values, and if it's valid part (bool)

    let mut char_points:Vec<Vec<char>> = Vec::new();
    let mut parts:Vec<Part>= Vec::new();

    let lines = stream.lines();
    for (y,l) in lines.enumerate(){
        let mut is_number=false;
        //
        let mut p:Vec<Point> = Vec::new();
        let mut num_str = "".to_string();

        let mut char_point:Vec<char> = Vec::new();
        for (x,c) in l.chars().enumerate(){

            char_point.push(c);

            if c.is_ascii_digit(){
                is_number = true;
                let p_ = Point{ x: x as i32, y: y as i32};
                p.push(p_);
                num_str.push_str(&c.to_string());
            }else{
                if is_number {
                    let part = Part{
                        num: num_str.parse::<u32>().unwrap_or(0),
                        valid: false,
                        p: p.clone() };

                    parts.push(part);
                    p.clear();
                    num_str ="".to_string();
                }
                is_number = false;
            }
        }
        char_points.push(char_point);

    }

    for i in &parts{
        println!("{:?}",i);
    }

    // . . .
    // . X .
    // . . .

    // [-1,-1] [0,-1] [1,-1]
    // [-1,0] X [1,0]
    // [-1,1] [0,1] [1,1]

    let around:Vec<[i32; 2]> = vec![[-1,-1],[0,-1],[1,-1],[-1,0],[1,0],[-1,1],[0,1],[1,1]];

    //
    // for i in parts.iter(){
    //     println!("{:?}",i);
    // }
    //
    let max_y = char_points.iter().len() as i32;
    let max_x = char_points.iter().collect::<Vec<_>>().iter().len() as i32;
    // println!("{} {}",max_x,max_y);
    //
    let mut valid_parts_sum = 0;
    for i in parts{
        'parts: for j in i.p{
            for k in around.iter(){
                let x = j.x + k[0] ;
                let y = j.y + k[1] ;
                if x>0 && y>0 && x<max_x && y<max_y{
                    
                    let p = Point{ x, y };
                    // println!("{} {}",x,y);
                    let ch = get_point(&char_points,p);
                    if !(ch.is_ascii_digit() || ch=='.'){
                        // ifch.is_ascii()){
                        // print!("{}",ch);
                        valid_parts_sum += i.num;
                        print!("{}{} ",i.num , ch);
                        break 'parts

                    // }else{

                    }
                }
            }
        }
    }
    println!("> {}",valid_parts_sum);

}

// 1429 too low
// 549292 too low

//549292


fn get_point(array2d: &Vec<Vec<char>> , point:Point ) -> char{
    let x = point.x as usize;
    let y = point.y as usize;
    let e = array2d.iter().collect::<Vec<_>>()[y];
    let f = e.iter().collect::<Vec<_>>()[x];
    *f
}








