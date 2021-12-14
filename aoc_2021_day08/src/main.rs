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

fn return_number(number: String) -> i32 {
    let num_len = number.len();
    match num_len {
        2=> 1,
        4=> 4,
        3=> 7,
        7=> 8,
        _=>-1
    }
}

fn decoder(coded: Vec<String>, decode: Vec<String>) -> i32 {
    let mut zero = "".to_string();
    let mut one = "".to_string();
    let mut two = "".to_string();
    let mut three = "".to_string();
    let mut four = "".to_string();
    let mut five = "".to_string();
    let mut six = "".to_string();
    let mut seven = "".to_string();
    let eight = "abcdefg".to_string();
    let mut nine = "".to_string();
    let mut five_counts = Vec::new();
    let mut six_counts = Vec::new();
    for value in coded {
        let mut sorted_value = value.chars().collect::<Vec<char>>();
        sorted_value.sort();
        match value.len() {
            2 => one = sorted_value.clone().iter().collect::<String>(),
            3 => seven = sorted_value.clone().iter().collect::<String>(),
            4 => four = sorted_value.clone().iter().collect::<String>(),
            5 => five_counts.push(sorted_value.clone().iter().collect::<String>()),
            6 => six_counts.push(sorted_value.clone().iter().collect::<String>()),
            _ => continue
        }
    }
    for option in six_counts.clone() {
        if option.chars().filter(|x| four.chars()
            .collect::<Vec<char>>().contains(x)).collect::<String>() == four {
            nine = option.clone();
        }
    }
    for option in six_counts.clone() {
        if option.chars().filter(|x| one.chars()
            .collect::<Vec<char>>().contains(x)).collect::<String>() == one
        && option != nine {
            zero = option.clone();
        }
    }
    for option in six_counts {
        if option != nine  && option != zero{
            six = option.clone();
        }
    }
    for option in five_counts.clone() {
        if option.chars().filter(|x| one.chars()
            .collect::<Vec<char>>().contains(x)).collect::<String>() == one {
            three = option.clone();
        }
    }
    for option in five_counts.clone() {
        if six.chars().filter(|x| option.chars()
            .collect::<Vec<char>>().contains(x)).collect::<String>() == option
        && option != three {
            five = option.clone();
        }
    }
    for option in five_counts {
        if option != three && option != five {
            two = option.clone();
        }
    }

    let mut builder = Vec::new();
    for value in decode {
        let mut sorted_value = value.chars().collect::<Vec<char>>();
        sorted_value.sort();
        let output = sorted_value.clone().iter().collect::<String>();
        if output == zero {builder.push('0')}
        else if output == one {builder.push('1')}
        else if output == two {builder.push('2')}
        else if output == three {builder.push('3')}
        else if output == four {builder.push('4')}
        else if output == five {builder.push('5')}
        else if output == six {builder.push('6')}
        else if output == seven {builder.push('7')}
        else if output == eight {builder.push('8')}
        else if output == nine {builder.push('9')}

    }

    builder.iter().collect::<String>().parse::<i32>().unwrap()

    }



fn star_1() {
    let input = read_a_file("C:/input/input08.txt").unwrap();
    let mut outputs_vec = Vec::new();
    for line in input {
        let right = &line.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>()[1];
        let outputs: Vec<String> = right.split(" ").map(|s| s.to_string()).collect();
        outputs_vec.push(outputs);
    }
    let mut total_good = 0;
    for outputs in outputs_vec.clone() {
        for output in outputs {
            if return_number(output) != -1 {
                total_good += 1;
            }
        }
    }
    println!("Easy Numbers: {}", total_good);
}

fn star_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut everything_vec = Vec::new();
    let mut outputs_vec = Vec::new();
    for line in input {
        let left = &line.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>()[0];
        let right = &line.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>()[1];
        let mut inputs: Vec<String> = left.split(" ").map(|s| s.to_string()).collect();
        let outputs: Vec<String> = right.split(" ").map(|s| s.to_string()).collect();
        inputs.append(&mut outputs.clone());
        everything_vec.push(inputs);
        outputs_vec.push(outputs);
    }
    let mut total = 0;
    for i in 0..everything_vec.len() {
        total+=decoder(everything_vec[i].clone(), outputs_vec[i].clone());
    }
    println!("Deduction Sucks: {}", total);

}



fn main() {
    star_1();
    star_2();
}
