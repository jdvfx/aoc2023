//
#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs;

// Part 2
fn hunum_to_num(line: &str) -> String {
    let n = "one two three four five six seven eight nine".to_string();
    let nums: Vec<String> = n.split(" ").map(|x| x.to_string()).collect();
    let mut s: String = line.to_string();
    for (idx, i) in nums.iter().enumerate() {
        if s.contains(i) {
            let x = (idx + 1).to_string();
            let ii = i.as_str();
            s = s.replace(ii, &x);
        }
    }
    s.to_string()
}

fn get_tok(line: &str) -> u32 {
    let n = "1 2 3 4 5 6 7 8 9 one two three four five six seven eight nine".to_string();
    let tok: Vec<String> = n.split(" ").map(|x| x.to_string()).collect();

    let mut x = 0;
    let mut toks: Vec<String> = Vec::new();
    let mut current_string: String = "".to_string();
    for _i in 0..line.len() {
        let c = &line[x..x + 1];
        current_string.push_str(c);
        'o: for j in &tok {
            if current_string.contains(j) {
                let n = hunum_to_num(j);
                toks.push(n);
                // keep the last char for sneaky twone or eighthree
                // IF it's not a number!

                current_string = current_string.chars().last().unwrap().to_string();
                match current_string.parse::<u32>() {
                    Ok(_) => {
                        current_string = "".to_string();
                    }
                    _ => (),
                }
                break 'o;
            }
        }
        x += 1
    }

    match toks.len() {
        0 => return 0,
        1 => {
            let mut a = toks.first().unwrap().to_owned();
            a.push_str(&a.to_owned());
            return a.parse::<u32>().unwrap();
        }
        _ => {
            let mut f = toks.first().unwrap().to_owned();
            let l = toks.last().unwrap().to_owned();
            f.push_str(&l);
            return f.parse::<u32>().unwrap();
        }
    }
}

fn main() {
    let stream = fs::read_to_string("./input1.txt").unwrap();
    let lines = stream.trim().split("\n").collect::<Vec<_>>();

    let mut sum: u32 = 0;
    for line in lines {
        let num = get_tok(&line);
        println!("{} {}", num, line);
        sum += num;
    }
    println!("TOTAL {}", sum);
}
