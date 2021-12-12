use std::fs;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Connection {
    start: String,
    end: String,
}

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

fn star_1_2() {
    let input = read_a_file("C:/input/input.txt").unwrap();
    let mut conn_vec = Vec::new();
    for line in input {
        let start = line
            .split("-")
            .collect::<Vec<&str>>()[0]
            .clone()
            .to_string();
        let end = line
            .split("-")
            .collect::<Vec<&str>>()[1]
            .clone()
            .to_string();
        let new_conn = Connection { start, end };
        conn_vec.push(new_conn);
    }
    let mut routes = Vec::new();
    dig_route("start".to_string(),
              &mut vec!["start".to_string()],
              &conn_vec, &mut routes,
    false);
    println!("Route count 1: {}", routes.len());

    routes = Vec::new();
    dig_route("start".to_string(),
              &mut vec!["start".to_string()],
              &conn_vec, &mut routes,
              true);
    println!("Route count 2: {}", routes.len());
}

fn is_lower(chs: &str) -> bool {
    let ch = chs.chars().collect::<Vec<char>>()[0];
    if ch as u32 >= 'a' as u32 && ch as u32 <= 'z' as u32 {
        return true
    }
    return false
}

fn dig_route(current: String,
             path: &mut Vec<String>,
             conn_vec: & Vec<Connection>,
             routes: &mut Vec<Vec<String>>,
             single_small: bool)
{
    if current == "end" { routes.push(path.clone()); return }
    for conn in conn_vec {
        if current == conn.start && conn.end != "start"
        && (!path.contains(&conn.end) || !is_lower(&conn.end) || single_small)
        {
            let ss = if path.contains(&conn.end) && is_lower(&conn.end) {false}
                else {single_small};
            path.push(conn.end.clone());
            dig_route(conn.end.clone(), path, conn_vec, routes, ss);
            path.pop();
        }
        else if current == conn.end && conn.start != "start"
        && (!path.contains(&conn.start) || !is_lower(&conn.start) || single_small)
        {
            let ss = if path.contains(&conn.start) && is_lower(&conn.start) {false}
                else {single_small};
            path.push(conn.start.clone());
            dig_route(conn.start.clone(), path, conn_vec, routes, ss);
            path.pop();
        }
    }
}


fn main() {
    star_1_2();
}
