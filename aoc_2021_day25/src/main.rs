use std::cmp::min;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn read_a_file(filename: &str) -> std::io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        //let pushed: i64 = line.unwrap().parse().unwrap();
        vec.push(line.unwrap());
    }
    return Ok(vec);
}

fn star_1(file: &str) {
    let input = read_a_file(file).unwrap();
    let mut map = Vec::new();
    for line in input {
        let chars = line.chars().collect::<Vec<char>>();
        map.push(chars);
    }
    let mut loops = 0;
    let mut new_points = Vec::new();
    let mut new_blank = Vec::new();
    'outer: loop {
        loops += 1;
        /*println!("Loop {}", loops);
        for line in &map {
            for char in line {
                print!("{}", char);
            }
            println!();
        }
        println!();*/
        let mut moved = false;
        for index1 in 0..map.len() {
            for index2 in 0..map[0].len() {
                if map[index1][index2] == '>' && index2 == map[0].len() - 1 && map[index1][0] == '.'
                {
                    new_points.push((index1, 0));
                    new_blank.push((index1, index2));
                    moved = true;
                } else if map[index1][index2] == '>'
                    && index2 < map[0].len() - 1
                    && map[index1][min(index2 + 1, map[0].len() - 1)] == '.'
                {
                    new_points.push((index1, index2 + 1));
                    new_blank.push((index1, index2));
                    moved = true;
                }
            }
        }
        for point in new_points {
            map[point.0][point.1] = '>';
        }
        for point in new_blank {
            map[point.0][point.1] = '.';
        }
        new_points = Vec::new();
        new_blank = Vec::new();
        for index1 in 0..map.len() {
            for index2 in 0..map[0].len() {
                if map[index1][index2] == 'v' && index1 == map.len() - 1 && map[0][index2] == '.' {
                    new_points.push((0, index2));
                    new_blank.push((index1, index2));
                    moved = true;
                } else if map[index1][index2] == 'v'
                    && index1 < map.len() - 1
                    && map[min(index1 + 1, map.len() - 1)][index2] == '.'
                {
                    new_points.push((index1 + 1, index2));
                    new_blank.push((index1, index2));
                    moved = true;
                }
            }
        }
        for point in new_points {
            map[point.0][point.1] = 'v';
        }
        for point in new_blank {
            map[point.0][point.1] = '.';
        }
        new_points = Vec::new();
        new_blank = Vec::new();
        if !moved {
            break 'outer;
        }
    }
    println!("Loops {}", loops);
}

fn main() {
    let present = ("122521").parse::<i32>();
    present.unwrap();
    star_1("C:/input/input25.txt");
}
