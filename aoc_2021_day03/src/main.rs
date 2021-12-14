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

fn rotate_2d_vector(input: Vec<Vec<u8>>) -> std::io::Result<Vec<Vec<u8>>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for (index1,line) in input.iter().enumerate() {
        for (index2, value) in line.iter().enumerate() {
            if index1 == 0 {
                result.push(Vec::new())
            }
            result[index2].push(*value);
        }
    }
    return Ok(result)
}

fn least_most_frequent(input: Vec<u8>) -> (u8, u8) {
    let mut zero_count = 0;
    let mut one_count = 0;
    let mut total_count = 0;
    for value in input {
        total_count += 1;
        if value == 0 {
            zero_count += 1;
        }
        else {
            one_count += 1;
        }
    }
    return
    if zero_count == total_count { (0,0) }
    else if one_count == total_count { (1,1) }
    else if one_count > zero_count { (1,0) }
    else if one_count < zero_count { (0,1) }
    else { (1,0) };
}

fn binary_string_to_decimal(input: &str) -> i64 {
    return i64::from_str_radix(input,2).unwrap()
}

fn star_1() {
    let input = read_a_file("C:/input/input03.txt").unwrap();
    let mut vec2d = Vec::new();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for line in input {
        let split: Vec<char> = line.chars().collect();
        let split_int: Vec<u8> = split.iter().map(|&x| if (x as u8) == 48 {0} else {1})
                                            .collect();
        vec2d.push(split_int);
    }
    for column in rotate_2d_vector(vec2d.clone()).unwrap() {
        let (gamma_addition, epsilon_addition) = least_most_frequent(column);
        gamma_rate += &gamma_addition.to_string();
        epsilon_rate += &epsilon_addition.to_string();
    }
    println!("Result is {}", binary_string_to_decimal(&gamma_rate)
                           * binary_string_to_decimal(&epsilon_rate));
}

fn star_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut vec2d = Vec::new();
    for line in input {
        let split: Vec<char> = line.chars().collect();
        let split_int: Vec<u8> = split.iter().map(|&x| if (x as u8) == 48 {0} else {1})
            .collect();
        vec2d.push(split_int);
    }
    let mut co2_scrub = vec2d.clone();
    let mut oxygen_gen = vec2d.clone();
    let mut ind = 0;
    while co2_scrub.len() > 1 {
        let check = rotate_2d_vector(co2_scrub.clone()).unwrap();
        let (maxed, _mined) = least_most_frequent(check[ind].clone());
        co2_scrub = co2_scrub.into_iter().filter(|x| x[ind] == maxed).collect::<Vec<Vec<u8>>>();
        ind += 1;
    }
    ind = 0;
    while oxygen_gen.len() > 1 {
        let check = rotate_2d_vector(oxygen_gen.clone()).unwrap();
        let (_maxed, mined) = least_most_frequent(check[ind].clone());
        oxygen_gen = oxygen_gen.into_iter().filter(|x| x[ind] == mined).collect::<Vec<Vec<u8>>>();
        ind += 1;
    }
    println!("Result is {}",
             binary_string_to_decimal(
                 &co2_scrub[0] .iter()
                                     .map(|x| x
                                     .to_string())
                                     .collect::<String>())
             *
             binary_string_to_decimal(
                 &oxygen_gen[0].iter()
                                     .map(|x| x
                                     .to_string())
                                     .collect::<String>()));

}


fn main() {
    star_1();
    star_2();
}
