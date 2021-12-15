extern crate pathfinding;

use pathfinding::prelude::dijkstra;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

type Coord = (i32, i32);

fn adjacent(point: &Coord, grid: &Vec<Vec<u32>>) -> Vec<(Coord, usize)> {
    let mut adjacent = Vec::new();
    if point.0 - 1 >= 0 {
        let p = point.0 - 1;
        adjacent.push((
            (point.0 - 1, point.1),
            grid[p as usize][point.1 as usize] as usize,
        ));
    }
    if point.0 + 1 <= grid[0].len() as i32 - 1 {
        let p = point.0 + 1;
        adjacent.push((
            (point.0 + 1, point.1),
            grid[p as usize][point.1 as usize] as usize,
        ));
    }
    if point.1 - 1 >= 0 {
        let p = point.1 - 1;
        adjacent.push((
            (point.0, point.1 - 1),
            grid[point.0 as usize][p as usize] as usize,
        ));
    }
    if point.1 + 1 <= grid.len() as i32 - 1 {
        let p = point.1 + 1;
        adjacent.push((
            (point.0, point.1 + 1),
            grid[point.0 as usize][p as usize] as usize,
        ));
    }
    adjacent
}

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

fn advancer(line: Vec<u32>) -> Vec<u32> {
    let mut temp_line = line.clone();
    for i in 0..temp_line.len() {
        temp_line[i] += 1;
        if temp_line[i] == 10 {
            temp_line[i] = 1;
        }
    }
    temp_line
}

fn expander(grid: &mut Vec<Vec<u32>>, times: u8) {
    for i in 0..grid.len() {
        let mut temp_line = grid[i].clone();
        for _ in 0..times - 1 {
            temp_line = advancer(temp_line);
            grid[i].extend(temp_line.clone());
        }
    }
    let o_len = grid.len();
    for a in 0..times - 1 {
        for i in (a as usize * o_len as usize)..((a as usize + 1) * o_len as usize) {
            let temp_line = advancer(grid[i].clone());
            grid.push(temp_line);
        }
    }
}

fn star_1_2(expand: bool) {
    let input = read_a_file("C:/input/input15.txt").unwrap();
    let mut grid = Vec::new();
    for line in input {
        let mut pusher = Vec::new();
        for chr in line.chars().collect::<Vec<char>>() {
            pusher.push(chr as u32 - '0' as u32);
        }
        grid.push(pusher);
    }
    if expand {
        expander(&mut grid, 5);
    }
    let start: Coord = (0, 0);
    let end: Coord = (grid[0].len() as i32 - 1, grid.len() as i32 - 1);
    let result = dijkstra(&start, |p| adjacent(p, &grid), |&p| p == end);
    println!("Result {:?}", result.unwrap().1);
}

fn main() {
    star_1_2(false);
    star_1_2(true);
}
