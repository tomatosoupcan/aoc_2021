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

fn all_nines(input: &Vec<Vec<i32>>) -> bool {
    for line in input {
        for pos in line {
            if pos != &9 {
                return false
            }
        }
    }
    return true
}

fn iter_basin_sum(y: usize, x: usize, map: &mut Vec<Vec<i32>>) -> i32 {
    let mut total_count = 1;
    //println!("Investigating point {},{}", x, y);
    map[y][x] = 9;
    if map[y][x+1] == 0 {
        total_count += iter_basin_sum(y, x+1, map);
    }
    if map[y+1][x] == 0 {
        total_count += iter_basin_sum(y+1, x, map);
    }
    if map[y][x-1] == 0 {
        total_count += iter_basin_sum(y, x-1, map);
    }
    if map[y-1][x] == 0 {
        total_count += iter_basin_sum(y-1, x, map);
    }
    total_count
}

fn star_1() {
    let input = read_a_file("C:/input/input09.txt").unwrap();
    let mut height_vec = Vec::new();
    let mut top_bottom = Vec::new();
    for _ in 0..input[0].len()+2 {
        top_bottom.push(9);
    }
    height_vec.push(top_bottom.clone());
    for line in input {
        let mut line_vec = Vec::new();
        line_vec.push(9);
        for chr in line.chars() {
            line_vec.push(chr as i32 - 48);
        }
        line_vec.push(9);
        height_vec.push(line_vec);
    }
    height_vec.push(top_bottom);
    let mut risk_level = 0;
    for x in 1..height_vec[0].len()-1 {
        for y in 1..height_vec.len()-1 {
            let current_pos = height_vec[y][x];
            if height_vec[y-1][x-1] > current_pos
                && height_vec[y][x-1] > current_pos
                && height_vec[y-1][x] > current_pos
                && height_vec[y+1][x-1] > current_pos
                && height_vec[y-1][x+1] > current_pos
                && height_vec[y+1][x+1] > current_pos
                && height_vec[y+1][x] > current_pos
                && height_vec[y][x+1] > current_pos {
                risk_level += current_pos + 1;
            }
        }
    }
    println!("Risk Level: {}", risk_level);
}

fn star_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut height_vec = Vec::new();
    let mut top_bottom = Vec::new();
    for _ in 0..input[0].len()+2 {
        top_bottom.push(9);
    }
    height_vec.push(top_bottom.clone());
    for line in input {
        let mut line_vec = Vec::new();
        line_vec.push(9);
        for chr in line.chars() {
            if chr as i32 - 48 == 9 {
                line_vec.push(9);
            }
            else {
                line_vec.push(0);
            }
        }
        line_vec.push(9);
        height_vec.push(line_vec);
    }
    height_vec.push(top_bottom);
    /*for line in &height_vec {
        println!("{:?}", line);
    }*/

    let mut basin_list = Vec::new();
    while !all_nines(&height_vec) {
        for x in 1..height_vec[0].len()-1 {
            for y in 1..height_vec.len()-1 {
                if height_vec[y][x] == 0 {
                    basin_list.push(iter_basin_sum(y,x, &mut height_vec));
                }
            }
        }
    }
    basin_list.sort();
    let three_basins = &basin_list[basin_list.len()-3..basin_list.len()];
    println!("Three Basin Product: {}", three_basins[0]*three_basins[1]*three_basins[2]);
}

fn main() {
    star_1();
    star_2();
}
