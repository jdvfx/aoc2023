#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
use std::{fs, str::FromStr};


#[derive(Debug)]
struct Game{
    id: u32,
    r: u32,
    g: u32,
    b: u32,
}


#[derive(Debug)]
struct GameError;

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.split_once(":").ok_or(GameError);
        let j = x.unwrap();
        let jj =  j.0.split_once(" ").ok_or(GameError);
        let id = jj.unwrap().1.parse::<u32>().unwrap();
        let xc = j.1.split(";").map(|x|x.trim()).collect::<Vec<_>>();
        println!("-- {:?}",xc);

        // let id = x.split_one(" ");
        Ok(Game{
            id,
            r: 0,
            g: 0,
            b: 0,
        })

    }
}




fn main() {


    // let stream = fs::read_to_string("./input.txt").unwrap();

    let stream = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


    for line in stream.lines(){
        println!("{:?}",line);
        let game:Game = Game::from_str(line).unwrap();
        println!("{:?}",game);
    }


}


