use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn on_line(&self, point: Point) -> bool {
        let run = self.end.x - self.start.x;
        let rise = self.end.y - self.start.y;
        let m = rise/run;
        let b = self.end.y-(m*self.end.x);
        return if point.y == m * point.x + b
            && point.y.round() as i32 >= cmp::min(self.start.y.round() as i32,self.end.y.round() as i32)
            && point.y.round() as i32 <= cmp::max(self.start.y.round() as i32,self.end.y.round() as i32)
            && point.x.round() as i32 >= cmp::min(self.start.x.round() as i32,self.end.x.round() as i32)
            && point.x.round() as i32 <= cmp::max(self.start.x.round() as i32,self.end.x.round() as i32)
        {
            true
        }
        else if run == 0.0 && self.start.x == point.x
            && point.y.round() as i32 >= cmp::min(self.start.y.round() as i32,self.end.y.round() as i32)
            && point.y.round() as i32 <= cmp::max(self.start.y.round() as i32,self.end.y.round() as i32)
            && point.x.round() as i32 >= cmp::min(self.start.x.round() as i32,self.end.x.round() as i32)
            && point.x.round() as i32 <= cmp::max(self.start.x.round() as i32,self.end.x.round() as i32)
        {
            true
        }
        else {
            false
        }
    }
    fn is_straight(&self) -> bool {
        return if self.end.x == self.start.x || self.end.y == self.start.y {
            true
        } else {
            false
        }
    }
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

fn star_1(straight: bool) {
    let input = read_a_file("C:/input/inputtest.txt").unwrap();
    let mut lines: Vec<Line> = Vec::new();
    for line in input {
        let point1g = line.split(" -> ").collect::<Vec<&str>>()[0];
        let point2g = line.split(" -> ").collect::<Vec<&str>>()[1];
        let point1x =
            point1g
                .split(",")
                .collect::<Vec<&str>>()[0]
                .parse::<f64>().unwrap();
        let point1y =
            point1g
                .split(",")
                .collect::<Vec<&str>>()[1]
                .parse::<f64>().unwrap();
        let point2x =
            point2g
                .split(",")
                .collect::<Vec<&str>>()[0]
                .parse::<f64>().unwrap();
        let point2y =
            point2g
                .split(",")
                .collect::<Vec<&str>>()[1]
                .parse::<f64>().unwrap();
        let point1 = {Point {x: point1x, y:point1y}};
        let point2 = {Point {x: point2x, y:point2y}};
        let line_made = {Line {start: point1, end: point2}};
        lines.push(line_made);
    }
    let mut max_x = -f64::INFINITY;
    let mut max_y = -f64::INFINITY;
    for line in &lines {
        if line.start.x > max_x {max_x = line.start.x}
        if line.start.y > max_y {max_y = line.start.y}
        if line.end.x > max_x {max_x = line.end.x}
        if line.end.y > max_y {max_y = line.end.y}
    }

    let mut result_vector = Vec::new();
    for column in 0..max_x.round() as i32 + 1 {
        let mut result_line = Vec::new();
        for row in 0..max_y.round() as i32 + 1 {
            let mut result = 0;
            for line in &lines {
                if (line.is_straight() || straight)
                    && line.on_line(Point {x: row as f64, y: column as f64}) {
                    result += 1;
                }
            }
            result_line.push(result);
        }
        result_vector.push(result_line)
    }
    let mut big_intersect = 0;
    for line in &result_vector {
        //println!("{:?}", line);
        big_intersect += line.iter().filter(|x| x > &&1).count();
    }
    println!("Number of >2 intersections is {}", big_intersect)


}

fn star_2() {
    star_1(true)
}

fn main() {
    star_1(false);
    star_2();
    /*let test_line = {Line {start: {Point {x: 7.0, y: 0.0}}, end: {Point {x: 7.0, y: 4.0}}}};
    let test_point = {Point {x: 7.0, y: 3.0}};
    println!("{}", test_line.on_line(test_point));*/
}
