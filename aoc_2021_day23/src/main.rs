use std::collections::HashMap;

fn manhattan(point1: (i32, i32), point2: (i32, i32)) -> i32 {
    return (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs();
}

fn check_finished(rooms: &Vec<Vec<char>>) -> bool {
    let mut result = true;
    for index in 0..4 {
        let char_check = (65 + index) as u8 as char;
        if rooms[index].iter().filter(|x| **x == char_check).count() != 4 {
            result = false;
        }
    }
    result
}

fn check_room(room: &Vec<char>, index: usize) -> bool {
    let mut result = true;
    let char_check = (65 + index) as u8 as char;
    if room.iter().filter(|x| **x == char_check).count() != 4 {
        result = false;
    }
    result
}

fn star_2() {
    let hallway = vec!['.', '.', '.', '.', '.', '.', '.'];
    let rooms = vec![
        vec!['D', 'D', 'D', 'B'],
        vec!['A', 'C', 'B', 'A'],
        vec!['B', 'B', 'A', 'D'],
        vec!['C', 'A', 'C', 'C'],
    ];
    let mut scores = Vec::new();
    rec_move(&hallway, &rooms, 0, &mut scores);
    println!("{:?}", scores);
}

fn rec_move(hw: &Vec<char>, rm: &Vec<Vec<char>>, score: u64, scores: &mut Vec<u64>) {
    let mut rooms = rm.clone();
    let mut hall = hw.clone();
    for index in 0..rooms.len() {
        let room = &mut rooms[index];
        if !check_room(&room, index) {
            let working_char = room.remove(0);
            room.insert(0, '.');
            for hw_index in 0..hall.len() {
                if index == 0 && hw_index == 0 && hw[1] == '.' && hw[0] == '.' {
                    hall[hw_index] = working_char;
                }
            }
        }
    }
    println!("{:?}", rooms);
}

fn main() {
    println!("I did star 1 by hand, it was 15516");
    println!("I gave up on part 2 and did it by hand too, it was 45272. Left remnants of what I was doing")
    //star_2(); //it's a no parsing kind of day
}
