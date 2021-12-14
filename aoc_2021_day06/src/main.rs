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

fn star_1(days: i64) {
    star_2(days);
}

fn star_2(days: i64) {
    let mut fish_days: Vec<u64> = vec![0;9];
    read_a_file("C:/input/input06.txt").unwrap()[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|fish| fish_days[fish] +=1);
    (0..days).for_each(|_days| { fish_days.rotate_left(1); fish_days[6] += fish_days[8]});
    println!("Fish Count {}", fish_days.iter().sum::<u64>());
}

fn main() {
    star_1(80);
    star_2(256);
}
