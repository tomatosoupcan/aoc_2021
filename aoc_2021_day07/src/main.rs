use std::fs;
use std::io::BufReader;
use std::io::BufRead;

fn read_a_file(filename: &str) -> std::io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        //let pushed: i64 = line.unwrap().parse().unwrap();
        vec.push(line.unwrap());
    }
    return Ok(vec)
}

fn median(vec: &Vec<i32>) -> i32 {
    let mut new_vec = vec.clone();
    new_vec.sort();
    new_vec[new_vec.len()/2]
}

fn mean(vec: &Vec<i32>) -> i32 {
    let new_vec = vec.clone();
    (new_vec.iter().sum::<i32>() as f32 / vec.len() as f32).floor() as i32
}

fn sum_up(number: i32) -> i32 {
    (number*(number+1))/2
}

fn star_1() {
    let input: Vec<i32> = read_a_file("C:/input/input.txt").unwrap()[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect();
    let med_val = median(&input);
    println!("Least fuel: {}",input.into_iter().map(|x| (x - med_val).abs()).sum::<i32>());
}

fn star_2() {
    let input: Vec<i32> = read_a_file("C:/input/input.txt").unwrap()[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect();
    let mean_val = mean(&input);
    println!("Least fuel: {}",input.into_iter().map(|x| sum_up((x - mean_val).abs())).sum::<i32>());
}

fn main() {
    star_1();
    star_2();
}
