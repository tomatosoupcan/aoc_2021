use std::collections::HashMap;
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

fn insert_or_add(hash_map: &mut HashMap<String, u64>, key:String, value: u64) {
    if hash_map.contains_key(&key) {
        *hash_map.get_mut(&key).unwrap() += value;
    }
    else {
        hash_map.insert(key, value);
    }
}

fn remove_or_sub(hash_map: &mut HashMap<String, u64>, key:String, value: u64) {
    if hash_map.contains_key(&key) {
        *hash_map.get_mut(&key).unwrap() -= value;
        if hash_map.get(&key).unwrap() == &0 {
            hash_map.remove(&key);
        }
    }
}

fn star_1_2(iters: i32) {
    let mut input = read_a_file("C:/input/input14.txt").unwrap();
    let init_template = input.remove(0);
    input.remove(0);
    let mut pair_swaps = Vec::new();
    for mapping in input {
        let (pair, insert) = mapping.split_once(" -> ").unwrap();
        let pair_chars = pair.chars().collect::<Vec<char>>();
        let new_left = pair_chars[0].to_string() + insert;
        let new_right = insert.to_string() + pair_chars[1].to_string().as_str();
        pair_swaps.push((pair.to_string(), new_left, new_right, insert.to_string()));
    }
    let mut char_counts = HashMap::new();
    for chr in init_template.chars().collect::<Vec<char>>() {
        insert_or_add(&mut char_counts, chr.to_string(),1);
    }
    let mut pattern_counts = HashMap::new();
    for index in 0..init_template.len()-1 {
        insert_or_add(&mut pattern_counts,
                      init_template.chars()
                          .collect::<Vec<char>>()[index].to_string()
                          +
                          &init_template.chars()
                          .collect::<Vec<char>>()[index+1].to_string(),
                      1)
    }
    let mut insert_counts = HashMap::new();
    let mut remove_counts = HashMap::new();
    for _ in 0..iters {
        for pair in &pair_swaps {
            if pattern_counts.contains_key(&pair.0) {
                let count = *pattern_counts.get(&pair.0).unwrap();
                insert_or_add(&mut remove_counts, pair.0.clone(), count);
                insert_or_add(&mut insert_counts, pair.1.clone(), count);
                insert_or_add(&mut insert_counts, pair.2.clone(), count);
                insert_or_add(&mut char_counts, pair.3.clone(), count);
            }
    }
        for hash in &remove_counts {
            remove_or_sub( &mut pattern_counts, hash.0.clone(), hash.1.clone())
        }
        for hash in &insert_counts {
            insert_or_add(&mut pattern_counts, hash.0.clone(), hash.1.clone());
        }
        remove_counts.clear();
        insert_counts.clear();
    }
    println!("{:?}", char_counts.values().max().unwrap() - char_counts.values().min().unwrap());
}

fn main() {
    star_1_2(10);
    star_1_2(40);
}