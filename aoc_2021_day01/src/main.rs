use std::fs;
use std::io::BufReader;
use std::io::BufRead;


fn read_a_file(filename: &str) -> std::io::Result<Vec<i64>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let pushed: i64 = line.unwrap().parse().unwrap();
        vec.push(pushed);
    }
    return Ok(vec)
}

fn star_1(input: Vec<i64>) {
    let mut count = 0;
    for slice in input.windows(2) {
        if slice[1] > slice[0] {
            count += 1;
        }
    }
    println!("Number of increases is {}", count);
}

fn star_2(input: Vec<i64>) {
    let mut compiled_sums: Vec<i64> = Vec::new();
    for slice in input.windows(3) {
        compiled_sums.push(slice.iter().sum());
    }
    star_1(compiled_sums)
}

fn main() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    star_1(input.clone());
    star_2(input.clone());
}
