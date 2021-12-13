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

fn rotate_2d_vector(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for (index1,line) in input.iter().enumerate() {
        for (index2, value) in line.iter().enumerate() {
            if index1 == 0 {
                result.push(Vec::new())
            }
            result[index2].push(*value);
        }
    }
    return result
}

fn star_1_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut coordinates = Vec::new();
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
            coordinates.push((x,y));
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
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for coord in &coordinates {
        if coord.0 < min_x {min_x = coord.0}
        if coord.0 > max_x {max_x = coord.0}
        if coord.1 < min_y {min_y = coord.1}
        if coord.1 > max_y {max_y = coord.1}
    }
    let mut grid = Vec::new();
    for y in min_y..max_y+1 {
        let mut line = Vec::new();
        for x in min_x..max_x+1 {
            let write = if coordinates.contains(&(x,y)) {1} else {0};
            line.push(write);
        }
        grid.push(line);
    }
    folder(grid, &folds);
}

fn folder(mut grid: Vec<Vec<i32>>, folds: &Vec<(char, i32)>) {
    let mut first_print = true;
    for fold in folds {
        if fold.0 == 'y' {
            grid = fold_y(grid, fold.1 as usize);
        } else {
            grid = fold_y(rotate_2d_vector(grid), fold.1 as usize);
            grid = rotate_2d_vector(rotate_2d_vector(rotate_2d_vector(grid)));
        }
        if first_print {
            first_print = false;
            let mut total = 0;
            for line in &grid {
                total+=line.iter().sum::<i32>();
            }
            println!("Total First Fold: {}", total);
        }
    }
    for line in &grid {
        println!("{:?}", line);
    }
}

fn fold_y(grid: Vec<Vec<i32>>, line: usize) -> Vec<Vec<i32>> {
    let mut stationary_grid = Vec::new();
    let mut folding_grid = Vec::new();
    let mut dummy_line = Vec::new();
    for _ in 0..grid[0].len() {
        dummy_line.push(0);
    }
    for x in 0..grid.len() {
        if x <= line {stationary_grid.push(grid[x].clone())}
        else {stationary_grid.push(dummy_line.clone())};

        if x > line {folding_grid.push(grid[x].clone())}
        else {folding_grid.push(dummy_line.clone())};
    }
    folding_grid.reverse();
    for x in 0..folding_grid.len() {
        for y in 0..folding_grid[0].len() {
            if folding_grid[x][y] == 1 {
                stationary_grid[x][y] = 1;
            }
        }
    }
    let mut return_grid = Vec::new();
    for x in 0..stationary_grid.len() {
        if x < line {
            return_grid.push(stationary_grid[x].clone());
        }
    }
    return return_grid;
}

fn main () {
    star_1_2();
}