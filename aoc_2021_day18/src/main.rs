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

fn star_1(input: Vec<String>) -> i64 {
    let mut final_line = "".to_string();
    for line in input {
        if final_line == "" {
            final_line = line.replace("[", "þ").replace("]", "ÿ");
        } else {
            //println!("Adding new line");
            final_line = "þ".to_string()
                + &final_line
                + ","
                + &line.replace("[", "þ").replace("]", "ÿ")
                + "ÿ";
            let mut lcv = final_line.chars().collect::<Vec<char>>();
            'outer: loop {
                //println!(
                //    "Current String: {:?}",
                //    &lcv.iter()
                //        .collect::<String>()
                //        .replace("ÿ", "]")
                //        .replace("þ", "[")
                //);
                let mut depth = 0;
                let mut side = false;
                let mut last_num = usize::MAX;
                let mut next_num = usize::MAX;
                for index in 0..lcv.len() - 1 {
                    let chr = lcv[index];
                    if depth > 4
                        && chr as u32 >= '0' as u32
                        && chr as u32 <= 'ý' as u32
                        && !side
                        && lcv[index + 1] == ','
                        && lcv[index + 2] as u32 >= '0' as u32
                        && lcv[index + 2] as u32 <= 'ý' as u32
                    {
                        //print!("Exploding        ");
                        //for _ in 0..index {
                        //    print!(" ")
                        //}
                        //print!("^..{}", index);
                        //println!();
                        let split_val_left = chr as u32 - '0' as u32;
                        let split_val_right = lcv[index + 2] as u32 - '0' as u32;
                        //look ahead until we see another number
                        'lookahead: for index2 in index + 3..lcv.len() - 1 {
                            if lcv[index2] as u32 >= '0' as u32 && lcv[index2] as u32 <= 'ý' as u32
                            {
                                next_num = index2;
                                break 'lookahead;
                            }
                        }
                        if last_num != usize::MAX {
                            lcv[last_num] = (lcv[last_num] as u32 + split_val_left) as u8 as char;
                        }
                        if next_num != usize::MAX {
                            lcv[next_num] = (lcv[next_num] as u32 + split_val_right) as u8 as char;
                        }
                        //remove the exploded pair
                        for _ in 0..4 {
                            lcv.remove(index - 1);
                        }
                        lcv[index - 1] = '0';
                        continue 'outer;
                    } else if chr == 'þ' {
                        depth += 1;
                        side = false;
                    } else if chr == 'ÿ' {
                        depth -= 1;
                    } else if chr == ',' {
                        side = true;
                    } else if chr as u32 >= '0' as u32 && chr as u32 <= 'ý' as u32 {
                        last_num = index;
                    }
                }
                for index in 0..lcv.len() - 1 {
                    let chr = lcv[index];
                    if index + 1 == (lcv.len() - 1) {
                        final_line = lcv.iter().collect::<String>();
                        break 'outer;
                    }
                    if chr as u32 >= ':' as u32 && chr as u32 <= 'ý' as u32 {
                        //print!("Splitting        ");
                        //for _ in 0..index {
                        //    print!(" ")
                        //}
                        //print!("^..{}", index);
                        //println!();
                        let left_val = ((chr as u32 - '0' as u32) as f32 / 2 as f32).floor();
                        let right_val = ((chr as u32 - '0' as u32) as f32 / 2 as f32).ceil();
                        let left_char = (left_val as u32 + '0' as u32) as u8 as char;
                        let right_char = (right_val as u32 + '0' as u32) as u8 as char;
                        let replace_string = "þ".to_string()
                            + left_char.to_string().as_str()
                            + ","
                            + right_char.to_string().as_str()
                            + "ÿ";
                        let mut temp_string = lcv.iter().collect::<String>();
                        temp_string = temp_string.as_str().replacen(
                            chr.to_string().as_str(),
                            &replace_string,
                            1,
                        );
                        lcv = temp_string.chars().collect::<Vec<char>>();
                        continue 'outer;
                    }
                }
            }
        }
    }
    final_line = final_line.replace("ÿ", "]").replace("þ", "[");
    //println!("Final Line {}", final_line);
    let mut int_vec = final_line
        .chars()
        .map(|x| {
            if x == '[' {
                -1
            } else if x == ']' {
                -2
            } else if x == ',' {
                -3
            } else {
                x as i64 - '0' as i64
            }
        })
        .collect::<Vec<i64>>();
    //println!("Vectorization {:?}", int_vec);
    'outer2: loop {
        for index in 0..int_vec.len() - 5 {
            if int_vec[index] == -1 && int_vec[index + 2] == -3 && int_vec[index + 4] == -2 {
                let x = int_vec[index + 1].clone();
                let y = int_vec[index + 3].clone();
                for _ in 0..4 {
                    int_vec.remove(index);
                }
                int_vec[index] = x * 3 + y * 2;
                continue 'outer2;
            }
        }
        break 'outer2;
    }
    return int_vec[1] * 3 + int_vec[3] * 2;
}

fn star_2(input: Vec<String>) -> i64 {
    let mut biggest_result = 0;
    for line in &input {
        for line2 in &input {
            if line != line2 {
                let mut temp_vec = Vec::new();
                temp_vec.push(line.clone());
                temp_vec.push(line2.clone());
                let result = star_1(temp_vec);
                if result > biggest_result {
                    biggest_result = result
                }
            }
        }
    }
    biggest_result
}

fn main() {
    println!(
        "Part 1: {}",
        star_1(read_a_file("C:/input/input18.txt").unwrap())
    );
    println!(
        "Part 2: {}",
        star_2(read_a_file("C:/input/input18.txt").unwrap())
    )
}
