use std::cmp::min;
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

fn star_1(file: &str) {
    let input = read_a_file(file).unwrap();
    let mut p1_position = input[0].split_once(": ").unwrap().1.parse::<i32>().unwrap();
    let mut p2_position = input[1].split_once(": ").unwrap().1.parse::<i32>().unwrap();
    let (mut die_val, mut die_rolls, mut p1_score, mut p2_score) = (100, 0, 0, 0);
    let mut player = false;
    while p1_score < 1000 && p2_score < 1000 {
        player = !player;
        for _ in 0..3 {
            die_val += 1;
            if die_val == 101 {
                die_val = 1;
            }
            if player {
                p1_position += die_val;
            } else {
                p2_position += die_val;
            }
        }
        p1_position = (p1_position - 1) % 10 + 1;
        p2_position = (p2_position - 1) % 10 + 1;
        if player {
            p1_score += p1_position;
        } else {
            p2_score += p2_position;
        }
        die_rolls += 3;
    }
    println!("{}", min(p1_score, p2_score) * die_rolls);
}

fn star_2(file: &str) {
    let input = read_a_file(file).unwrap();
    let p1_position = input[0].split_once(": ").unwrap().1.parse::<i32>().unwrap();
    let p2_position = input[1].split_once(": ").unwrap().1.parse::<i32>().unwrap();
    let roll_combos = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let (p1_wins, p2_wins) = process_game(p1_position, p2_position, &roll_combos, 0, 0, true);
    println!("P1 Wins: {}, P2 Wins: {}", p1_wins, p2_wins);
}

fn process_game(
    p1_pos: i32,
    p2_pos: i32,
    roll_combos: &Vec<(i32, i32)>,
    p1_score: i32,
    p2_score: i32,
    turn: bool,
) -> (u64, u64) {
    if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }
    let (mut wins1, mut wins2) = (0, 0);
    let (mut p1_temp, mut p2_temp, mut p1_temp_score, mut p2_temp_score);
    for moves in roll_combos {
        if turn {
            p1_temp = p1_pos + moves.0;
            p2_temp = p2_pos;
        } else {
            p1_temp = p1_pos;
            p2_temp = p2_pos + moves.0;
        }
        p1_temp = (p1_temp - 1) % 10 + 1;
        p2_temp = (p2_temp - 1) % 10 + 1;
        if turn {
            p1_temp_score = p1_score + p1_temp;
            p2_temp_score = p2_score;
        } else {
            p2_temp_score = p2_score + p2_temp;
            p1_temp_score = p1_score;
        }
        let (temp_w1, temp_w2) = process_game(
            p1_temp,
            p2_temp,
            roll_combos,
            p1_temp_score,
            p2_temp_score,
            !turn,
        );
        wins1 = wins1 + temp_w1 * moves.1 as u64;
        wins2 = wins2 + temp_w2 * moves.1 as u64;
    }
    return (wins1, wins2);
}

fn main() {
    star_1("C:/input/input21.txt");
    star_2("C:/input/input21.txt");
}
