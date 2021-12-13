use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

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

fn star_1_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut coordinates = HashSet::new();
    let mut folds = Vec::new();
    let mut coordinating = true;
    for line in input {
        if line == "" {coordinating = false;}
        else if coordinating {
            let (x,y) = line
                .split_once(",")
                .map(|x| (x.0.parse::<i32>().unwrap()
                          ,x.1.parse::<i32>().unwrap()))
                .unwrap();
            coordinates.insert((x,y));
        }
        else {
            let temp_line = line.replace("fold along ","");
            let (x,y) = temp_line
                .split_once("=")
                .map(|x| (x.0.to_string().chars().collect::<Vec<char>>()[0]
                          ,x.1.parse::<i32>().unwrap()))
                .unwrap();
            folds.push((x,y));
        }
    }
    let mut first_fold = true;
    for fold in folds {
        for coord in coordinates.clone() {
            if fold.0 == 'y' && coord.1 > fold.1 {
                coordinates.remove(&coord);
                coordinates.insert((coord.0, fold.1-((coord.1-fold.1)%(fold.1+1))));
            }
            if fold.0 == 'x' && coord.0 > fold.1 {
                coordinates.remove(&coord);
                coordinates.insert((fold.1-((coord.0-fold.1)%(fold.1+1)), coord.1));
            }
        }
        if first_fold {
            first_fold = false;
            println!("First Fold Count: {}", coordinates.len());
        }
    }
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for coord in &coordinates {
        if coord.0 > max_x {max_x = coord.0}
        if coord.1 > max_y {max_y = coord.1}
    }
    for y in 0..(max_y+1) as usize {
        for x in 0..(max_x+1) as usize {
            let mut print = ' ';
            for coord in &coordinates {
                if coord == &(x as i32, y as i32) {
                    print = '#'
                }
            }
            print!("{}", print);
        }
        println!();
    }
}

fn main () {
    star_1_2();
}