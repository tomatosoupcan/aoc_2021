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

fn pad_image(image: &mut Vec<Vec<u8>>, inf: u8) {
    let mut temp_vec = Vec::new();
    for _ in 0..image[0].len() {
        temp_vec.push(inf);
    }
    image.insert(0, temp_vec.clone());
    image.insert(0, temp_vec.clone());
    image.push(temp_vec.clone());
    image.push(temp_vec);
    for line in image {
        line.insert(0, inf);
        line.insert(0, inf);
        line.push(inf);
        line.push(inf);
    }
}

fn star_1_2(file: &str, iterations: i32) {
    let input = read_a_file(file).unwrap();
    let mut translator = Vec::new();
    let mut image = Vec::new();
    let mut infinite = 0;
    for line in input {
        if line != "" {
            let temp = line
                .chars()
                .map(|x| if x == '.' { 0 } else { 1 })
                .collect::<Vec<u8>>();
            if translator == Vec::new() {
                translator = temp;
            } else {
                image.push(temp);
            }
        }
    }
    println!("Translator: {:?}", translator);
    for _ in 0..iterations {
        pad_image(&mut image, infinite);
        if translator[0] == 1 {
            infinite = 1 - infinite;
        }
        let mut new_image = image
            .clone()
            .iter()
            .map(|x| x.iter().map(|x| infinite * 1 + 0 * x).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        for y in 1..image.len() - 1 {
            for x in 1..image[0].len() - 1 {
                let mut binary = Vec::new();
                for yy in 0..3 {
                    for xx in 0..3 {
                        binary.push(image[y + yy - 1][x + xx - 1])
                    }
                }
                let string_temp = binary
                    .iter()
                    .map(|x| if x == &0 { '0' } else { '1' })
                    .collect::<String>();
                let binary_out = isize::from_str_radix(&string_temp, 2).unwrap();
                new_image[y][x] = translator[binary_out as usize];
            }
        }
        image = new_image;
    }
    /*for line in &image {
        println!(
            "{:?}",
            line.iter()
                .map(|x| if x == &0 { '.' } else { '#' })
                .collect::<String>()
        );
    }*/
    println!(
        "{}",
        image.iter().fold(0 as i32, |acc, x| acc
            + x.iter().fold(0 as i32, |acc, y| acc + *y as i32))
    );
}

fn main() {
    star_1_2("C:/input/input20.txt", 2);
    star_1_2("C:/input/input20.txt", 50);
}
