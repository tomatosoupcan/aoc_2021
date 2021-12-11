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

fn star_1_2(stop_point: i32) {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut energy_grid = Vec::new();
    let mut top_bottom = Vec::new();
    for _ in 0..input[0].len()+2 {
        top_bottom.push(-3);
    }
    energy_grid.push(top_bottom.clone());
    for line in input {
        let mut char_line_vec = Vec::new();
        char_line_vec.push(-3);
        let char_line = line
                .chars()
                .map(|x| x as i32 - '0' as i32)
                .collect::<Vec<i32>>();
        for value in char_line {
            char_line_vec.push(value);
        }
        char_line_vec.push(-3);
        energy_grid.push(char_line_vec);
    }
    energy_grid.push(top_bottom);
    let mut total_flash = 0;
    let mut flash_step = 0;
    let mut step = 0;
    while flash_step == 0 {
        step+=1;
        let(e_flash,full_flash) = step_it(&mut energy_grid);
        total_flash += e_flash;
        if step==stop_point-1 {
            println!("Flash Count: {}\n", total_flash);
        }
        if full_flash && flash_step == 0 {
            flash_step = step;
        }
    };
    println!("Flash Step: {}", flash_step)
}

fn step_it(grid: &mut Vec<Vec<i32>>) -> (i32, bool) {
    let mut flash_count = 0;
    //increase all values by 1
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            grid[x][y] += 1;
        }
    }
    let mut still_flashing = true;

    while still_flashing {
        let mut flashed = false;
        //turn any flashed octopi into -1
        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                if grid[x][y] >= 9 {
                    grid[x][y] = -1;
                    flashed = true;
                    flash_count += 1;
                    if grid[x - 1][y - 1] >= 0 { grid[x - 1][y - 1] += 1 }
                    if grid[x][y - 1] >= 0 { grid[x][y - 1] += 1 }
                    if grid[x - 1][y] >= 0 { grid[x - 1][y] += 1 }
                    if grid[x + 1][y - 1] >= 0 { grid[x + 1][y - 1] += 1 }
                    if grid[x - 1][y + 1] >= 0 { grid[x - 1][y + 1] += 1 }
                    if grid[x + 1][y + 1] >= 0 { grid[x + 1][y + 1] += 1 }
                    if grid[x + 1][y] >= 0 { grid[x + 1][y] += 1 }
                    if grid[x][y + 1] >= 0 { grid[x][y + 1] += 1 }
                }
            }
        }
        if !flashed {
            still_flashing = false;
        }
    }
    //reset flashed octopi
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[x][y] == -2 {
                grid[x][y] = 0;
            }
        }
    }
    //check for a full flash
    let mut returnbool = true;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[x][y] != 0 {
                returnbool = false;
            }
        }
    }

    (flash_count, returnbool)
}

fn main() {
    star_1_2(100)
}
