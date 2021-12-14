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

fn star_1(modifier: &str) {
    let input = read_a_file("C:/input/input02.txt").unwrap();
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for pair in input {
        let direction = pair.split(" ").collect::<Vec<&str>>()[0];
        let amount: i64 = pair.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        match format!("{}{}", direction, modifier).as_str() {
            "forward"=>distance+=amount,
            "down"=>depth+=amount,
            "up"=>depth-=amount,
            "forward2"=>{distance+=amount; depth+=aim*amount},
            "down2"=>aim+=amount,
            "up2"=>aim-=amount,
            _=>println!("Oops"),
        }
    }
    println!("Result is {}", depth * distance);
}

fn star_2() {
    star_1("2");
}

fn main() {
    star_1("");
    star_2();
}
