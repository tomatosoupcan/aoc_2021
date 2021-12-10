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

fn all_one_way (input: String, result: bool) -> bool {
    let filtered = input.clone()
        .chars()
        .filter(|x| "[{(<".contains(x.clone()))
        .collect::<String>();
    let filtered2 = input.clone()
        .chars()
        .filter(|x| "]})>".contains(x.clone()))
        .collect::<String>();
    if filtered == input || filtered2 == input{
        result
    }
    else {
        !result
    }
}

fn find_mismatch (input: String) -> char {
    let stv = input.chars().collect::<Vec<char>>();
    for index in 0..stv.len()-1 {
        if "[{(<".contains(stv[index]) && "]})>".contains(stv[index+1]) {
            return stv[index+1];
        }
    }
    '?'
}

fn star_1_2() {
    let mut input = read_a_file("C:/input/input.txt").unwrap();
    for index in 0..input.len() {
        let mut replacing = true;
        while replacing {
            let temp_line = input[index].clone();
            input[index] = input[index]
                .replace("()", "")
                .replace("[]", "")
                .replace("{}", "")
                .replace("<>", "");
            if input[index] == temp_line {
                replacing = false;
            }
        }
    }
    let corrupted = input
        .iter()
        .filter(|x| all_one_way(x.to_string(),false))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let incomplete = input
        .iter()
        .filter(|x| all_one_way(x.to_string(),true))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut total_score: u64 = 0;
    for line in corrupted {
        total_score += match find_mismatch(line) {
            '(' | ')' => 3,
            '[' | ']' => 57,
            '{' | '}' => 1197,
            '<' | '>' => 25137,
            _=> 0
        }
    }
    println!("Corrupted Total Score is {}", total_score);

    let mut total_scores = Vec::new();
    for line in incomplete {
        total_score = 0;
        let mut line_vec = line.chars().collect::<Vec<char>>();
        while line_vec != vec![] {
            total_score *= 5;
            let matcher = line_vec.pop().unwrap();
            total_score += match matcher {
                '(' | ')' => 1,
                '[' | ']' => 2,
                '{' | '}' => 3,
                '<' | '>' => 4,
                _ => 0
            }
        }
        total_scores.push(total_score);
    }
    total_scores.sort();
    println!("Incomplete Score: {}", total_scores[total_scores.len()/2]);
}

fn main() {
    star_1_2();
}
