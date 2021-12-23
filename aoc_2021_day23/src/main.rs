use std::collections::HashMap;

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
    if check_finished(rm) {
        scores.push(score);
        return;
    }
    let mut rooms = rm.clone();
    let mut hall = hw.clone();
    for index in 0..rooms.len() {
        let room = &mut rooms[index];
        if !check_room(&room, index) {
            for space in available_spaces_from_room(&hall, index) {
                hall[space] = room.remove(0);
                //TODO: Find the absolute distance of the move
                rec_move(
                    &hall,
                    &rooms,
                    score + value_from_char(hall[space], 0),
                    scores,
                );
            }
        }
    }
    return;
}

fn value_from_char(ch: char, dist: u64) -> u64 {
    dist * match ch {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

fn available_spaces_from_room(hw_t: &Vec<char>, room: usize) -> Vec<usize> {
    let mut spaces = Vec::new();
    let hw = hw_t.iter().map(|x| *x == '.').collect::<Vec<bool>>();
    spaces.push(7); // use something ooi to indicate checking the final location move
    for hw_index in 0..hw.len() {
        if room == 0 {
            if hw[0] && hw[1] {
                spaces.push(0);
            }
            if hw[1] {
                spaces.push(1);
            }
            if hw[2] {
                spaces.push(2);
            }
            if hw[3] && hw[2] {
                spaces.push(3);
            }
            if hw[4] && hw[3] && hw[2] {
                spaces.push(4);
            }
            if hw[5] && hw[4] && hw[3] && hw[2] {
                spaces.push(5);
            }
            if hw[6] && hw[5] && hw[4] && hw[3] && hw[2] {
                spaces.push(6)
            }
        }
        if room == 1 {
            if hw[0] && hw[1] && hw[2] {
                spaces.push(0)
            }
            if hw[1] && hw[2] {
                spaces.push(1)
            }
            if hw[2] {
                spaces.push(2);
            }
            if hw[3] {
                spaces.push(3);
            }
            if hw[4] && hw[3] {
                spaces.push(4);
            }
            if hw[5] && hw[4] && hw[3] {
                spaces.push(5);
            }
            if hw[6] && hw[5] && hw[4] && hw[3] {
                spaces.push(4);
            }
        }
        if room == 2 {
            if hw[0] && hw[1] && hw[2] && hw[3] {
                spaces.push(0);
            }
            if hw[1] && hw[2] && hw[3] {
                spaces.push(1);
            }
            if hw[2] && hw[3] {
                spaces.push(2);
            }
            if hw[3] {
                spaces.push(3);
            }
            if hw[4] {
                spaces.push(4);
            }
            if hw[4] && hw[5] {
                spaces.push(5);
            }
            if hw[4] && hw[5] && hw[6] {
                spaces.push(6);
            }
        }
        if room == 3 {
            if hw[0] && hw[1] && hw[2] && hw[3] && hw[4] {
                spaces.push(0);
            }
            if hw[1] && hw[2] && hw[3] && hw[4] {
                spaces.push(1);
            }
            if hw[2] && hw[3] && hw[4] {
                spaces.push(2);
            }
            if hw[3] && hw[4] {
                spaces.push(3);
            }
            if hw[4] {
                spaces.push(4);
            }
            if hw[5] {
                spaces.push(5);
            }
            if hw[5] && hw[6] {
                spaces.push(6);
            }
        }
    }
    return vec![0];
}

fn main() {
    println!("I did star 1 by hand, it was 15516");
    println!("I gave up on part 2 and did it by hand too, it was 45272. Left remnants of what I was doing")
    //star_2(); //it's a no parsing kind of day
}
