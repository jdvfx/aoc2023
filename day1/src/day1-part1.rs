use std::fs;

fn get_num(line: &str) -> u32 {
    // Part 1
    // let n = "0123456789";
    // let nums: Vec<String> = n.chars().map(|x| x.to_string()).collect();

    // Part 2
    let n = "one two three four five six seven eight nine".to_string();
    let nums: Vec<String> = n.split(" ").map(|x| x.to_string()).collect();

    let x = line
        .chars()
        .into_iter()
        .filter(|x| nums.contains(&x.to_string()))
        .collect::<Vec<_>>();

    match x.len() {
        0 => return 0,
        1 => {
            let mut y = x.clone().first().unwrap().to_string();
            y.push_str(&y.clone());
            y.parse::<u32>().unwrap()
        }
        _ => {
            let mut first = x.clone().first().unwrap().to_string();
            let last = x.last().unwrap().to_string();
            first.push_str(&last);
            first.parse::<u32>().unwrap()
        }
    }
}

fn main() {
    let stream = fs::read_to_string("./input1.txt").unwrap();
    let lines = stream.trim().split("\n").collect::<Vec<_>>();

    let mut sum: u32 = 0;
    for line in lines {
        let num = get_num(&line);
        println!("{} {}", num, line);
        sum += num;
    }
    println!("{}", sum);
}
