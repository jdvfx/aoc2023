#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

fn main() {


    let stream = fs::read_to_string("./input.txt").unwrap();

//     let stream = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[derive(Debug)]
    struct Game{
        id: u32,
        pieces: u32
    }

    #[derive(Debug)]
    struct Cubes{
        id: u32,
        red: u32,
        green: u32,
        blue: u32
    }


    let mut id_sum = 0;
    for line in stream.lines(){
        let a = line.split_once(":").unwrap();

        let game_id = a.0.split_once(" ").unwrap().1;

        let id:u32  = game_id.parse().unwrap();

        let turns = a.1.split(";").collect::<Vec<_>>();
        let mut cubes = Cubes{ id, red: 0, green: 0, blue: 0 };
        for i in turns{
            let j = i.split(",").collect::<Vec<_>>();
            for k in j{
                let o = k.split(" ").collect::<Vec<_>>();
                let num = o[1].parse::<u32>().unwrap();
                let col = o[2];

                match col {
                    "red" => cubes.red = cubes.red.max(num),
                    "green" => cubes.green = cubes.green.max(num),
                    "blue" => cubes.blue = cubes.blue.max(num),
                    _ => ()

                }
            }

        }

        if !(cubes.red >12 || cubes.green >13  || cubes.blue > 14){
            id_sum += cubes.id;
        }


    }

println!("> {:?}",id_sum);

}


