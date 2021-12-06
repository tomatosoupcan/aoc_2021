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
    let input = read_a_file("C:/input/input.txt").unwrap();
    let school: Vec<usize> = input[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut fish_days: Vec<u64> = vec![0,0,0,0,0,0,0,0,0];
    for fish in school {
        fish_days[fish] += 1;
    }
    for _day in 1..days+1 {
        let push_it = fish_days.remove(0);
        fish_days.push(push_it);
        fish_days[6] += push_it;
    }
    println!("Fish Count {}", fish_days.iter().sum::<u64>());

}

fn main() {
    star_1(80);
    star_2(256);
}
