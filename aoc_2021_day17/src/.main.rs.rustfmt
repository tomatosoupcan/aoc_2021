use std::cmp::max;
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

fn star_1_2(range: isize) {
    let input = read_a_file("C:/input/input17.txt").unwrap()[0]
        .clone()
        .replace("target area: ", "")
        .replace("x=", "")
        .replace("y=", "");
    let (x_range, y_range) = input.split_once(", ").unwrap();
    let xr: (isize, isize) = x_range
        .split_once("..")
        .map(|x| (x.0.parse::<isize>().unwrap(), x.1.parse::<isize>().unwrap()))
        .unwrap();
    let yr: (isize, isize) = y_range
        .split_once("..")
        .map(|x| (x.0.parse::<isize>().unwrap(), x.1.parse::<isize>().unwrap()))
        .unwrap();
    println!("{:?} {:?}", xr, yr);
    let mut possible_options = 0;
    let mut true_max_y = isize::MIN;
    for x in 0..range {
        for y in (range * -1)..range {
            let mut added = false;
            let mut x_velocity = x;
            let mut y_velocity = y;
            let mut pos = (0, 0);
            let mut temp_max_y = isize::MIN;
            loop {
                pos.0 += x_velocity;
                pos.1 += y_velocity;
                if pos.1 > temp_max_y {
                    temp_max_y = pos.1.clone();
                }
                if pos.0 >= xr.0 && pos.0 <= xr.1 && pos.1 >= yr.0 && pos.1 <= yr.1 {
                    if temp_max_y > true_max_y {
                        true_max_y = temp_max_y.clone();
                    }
                    if !added {
                        possible_options += 1;
                        added = true;
                    }
                } else if pos.0 > xr.1 || pos.1 < yr.0 {
                    break;
                }
                x_velocity = max(x_velocity - 1, 0);
                y_velocity = y_velocity - 1;
            }
        }
    }
    println!("Max Y: {}", true_max_y);
    println!("Possible Options: {}", possible_options);
}

fn main() {
    star_1_2(300);
}
