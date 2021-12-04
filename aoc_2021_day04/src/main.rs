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

fn cross_off_values(board_list: Vec<Vec<Vec<i64>>>, number: i64) -> Vec<Vec<Vec<i64>>> {
    let mut new_boards = Vec::new();
    for board in board_list {
        let mut new_board = Vec::new();
        for line in board {
            new_board.push(line.into_iter().map(|x| if x == number {(-1 * x) - 1} else {x}).collect::<Vec<i64>>());
        }
        new_boards.push(new_board);
    }
    return new_boards
}

fn rotate_2d_vector(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();
    for (index1,line) in input.iter().enumerate() {
        for (index2, value) in line.iter().enumerate() {
            if index1 == 0 {
                result.push(Vec::new())
            }
            result[index2].push(*value);
        }
    }
    return result
}

fn check_for_win(board_list: Vec<Vec<Vec<i64>>>, number: i64) -> (bool, i64, Vec<Vec<i64>>) {
    let mut board_won = false;
    for board in board_list {
        //println!("Original:\n{:?}\n\nRotated:\n{:?}\n\n", board, rotate_2d_vector(board.clone()));
        for line in board.clone() {
            let filter_line = line.clone().into_iter().filter(|x| x < &0).collect::<Vec<i64>>();
            if filter_line == line {
                board_won = true;
            }
        }
        for line in rotate_2d_vector(board.clone()) {
            let filter_line = line.clone().into_iter().filter(|x| x < &0).collect::<Vec<i64>>();
            if filter_line == line {
                board_won = true;
            }
        }
        if board_won == true {
            let mut filter_sum = 0;
            for line in board.clone() {
                for num in line {
                    if num > 0 {filter_sum += num;}
                }
            }
            return (true, filter_sum*number, board.clone())
        }
    }
    return (false, 0, Vec::new())
}

fn star_1_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut number_order = Vec::new();
    let mut board = Vec::new();
    let mut bingo_boards: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut index: i64 = -1;
    for line in input {
        if number_order == Vec::new() {
            number_order = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
        }
        else if line == String::new() {
            if index > -1 {bingo_boards.push(board);}
            board = Vec::new();
            index += 1;
        }
        else {
            board.push(line.replace("  "," ")
                .split(" ")
                .filter(|x| x != &"")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>());
        }
    }
    bingo_boards.push(board);

    for number in number_order {
        let mut repeat = true;
        while repeat {
            bingo_boards = cross_off_values(bingo_boards.clone(), number);
            let (winner, score, boarded) = check_for_win(bingo_boards.clone(), number);
            if winner {
                println! {"Winner! Score is {}", score};
            }
            bingo_boards = bingo_boards.clone().into_iter().filter(|x| x != &boarded).collect();
            if bingo_boards.iter().count() == 0 {
                return
            }
            if !winner {
                repeat = false;
            }
            //println!("{:?}", bingo_boards);
        }
    }
}

fn main() {
    star_1_2();
}
