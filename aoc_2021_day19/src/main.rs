use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Scanner {
    points: Vec<Point>,
    position: Point,
    position_locked: bool,
}

fn get_transforms(the_scanner: &Scanner) -> Vec<Vec<Point>> {
    let scanner = &the_scanner.points;
    let mut transforms = Vec::new();
    for a in 0..24 {
        match a {
            0 => transforms.push(scanner.clone()),
            1 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.x,
                        y: -x.z,
                        z: x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            2 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.x,
                        y: -x.y,
                        z: -x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),

            3 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.x,
                        y: x.z,
                        z: -x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            4 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.y,
                        y: x.x,
                        z: x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            5 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.z,
                        y: x.x,
                        z: x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            6 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.y,
                        y: x.x,
                        z: -x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            7 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.z,
                        y: x.x,
                        z: -x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            8 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.x,
                        y: -x.y,
                        z: x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            9 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.x,
                        y: -x.z,
                        z: -x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            10 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.x,
                        y: x.y,
                        z: -x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            11 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.x,
                        y: x.z,
                        z: x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            12 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.y,
                        y: -x.x,
                        z: x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            13 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.z,
                        y: -x.x,
                        z: -x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            14 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.y,
                        y: -x.x,
                        z: -x.z,
                    })
                    .collect::<Vec<Point>>(),
            ),
            15 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.z,
                        y: -x.x,
                        z: x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            16 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.z,
                        y: x.y,
                        z: x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            17 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.y,
                        y: x.z,
                        z: x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            18 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.z,
                        y: -x.y,
                        z: x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            19 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.y,
                        y: -x.z,
                        z: x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            20 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.z,
                        y: -x.y,
                        z: -x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            21 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: -x.y,
                        y: x.z,
                        z: -x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            22 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.z,
                        y: x.y,
                        z: -x.x,
                    })
                    .collect::<Vec<Point>>(),
            ),
            23 => transforms.push(
                scanner
                    .iter()
                    .map(|x| Point {
                        x: x.y,
                        y: -x.z,
                        z: -x.y,
                    })
                    .collect::<Vec<Point>>(),
            ),
            _ => continue,
        }
    }
    transforms
}

fn vec_fr_point(points: &Vec<Point>, base: &Point) -> Vec<(i32, i32, i32)> {
    let mut distances = Vec::new();
    for point in points {
        if point != base {
            let x = point.x - base.x;
            let y = point.y - base.y;
            let z = point.z - base.z;
            distances.push((x, y, z));
        }
    }
    distances
}

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

fn star_1(input: Vec<String>) {
    let mut scanners = Vec::new();
    let mut current_scanner = Vec::new();
    for line in input {
        if line.len() == 0 {
            continue;
        } else if line.chars().collect::<Vec<char>>()[1] == '-' {
            scanners.push(Scanner {
                points: current_scanner,
                position: Point { x: 0, y: 0, z: 0 },
                position_locked: false,
            });
            current_scanner = Vec::new();
        } else {
            let split = line
                .split(",")
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            current_scanner.push(Point {
                x: split[0],
                y: split[1],
                z: split[2],
            });
        }
    }
    scanners.push(Scanner {
        points: current_scanner,
        position: Point { x: 0, y: 0, z: 0 },
        position_locked: false,
    });
    scanners.remove(0);
    scanners[0].position_locked = true;
    'outer: loop {
        let mut all_locked = true;
        for scanner in &scanners {
            if scanner.position_locked == false {
                all_locked = false;
            }
        }
        if all_locked == true {
            break 'outer;
        }
        for index in 0..scanners.len() - 1 {
            println!("Checking against {}", index);
            let scanner = scanners[index].clone();
            for scanner2 in &mut scanners {
                if scanner == *scanner2 {
                    continue;
                }
                if scanner2.position_locked {
                    continue;
                }
                for transform in get_transforms(&scanner2) {
                    let mut distance_count = 0;
                    let mut hash = HashSet::new();
                    for point in &scanner.points {
                        for point2 in &transform {
                            for distance in vec_fr_point(&transform, &point2) {
                                for distance2 in vec_fr_point(&scanner.points, &point) {
                                    if distance == distance2 {
                                        hash.insert((point, point2));
                                        distance_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    if distance_count >= 11 {
                        println!("Overlapped with Scanner {}: {:?}", index, hash);
                        println!("Scanner {}: Position {:?}", index, scanner.position);
                        let single_compare = hash.into_iter().collect::<Vec<(&Point, &Point)>>()[0];
                        let x_dif = single_compare.0.x - single_compare.1.x;
                        let y_dif = single_compare.0.y - single_compare.1.y;
                        let z_dif = single_compare.0.z - single_compare.1.z;
                        if !scanner2.position_locked {
                            scanner2.position_locked = true;
                            scanner2.position.x += x_dif;
                            scanner2.position.x += scanner.position.x;
                            scanner2.position.y += y_dif;
                            scanner2.position.y += scanner.position.y;
                            scanner2.position.z += z_dif;
                            scanner2.position.z += scanner.position.z;
                            scanner2.points = transform;
                        }
                    }
                }
            }
        }
    }
    for scanner in &scanners {
        println!("{:?}", scanner.position)
    }
    let mut total_points = HashSet::new();
    for scanner in &scanners {
        for point in &scanner.points {
            let mut temp_point = point.clone();
            temp_point.x += scanner.position.x;
            temp_point.y += scanner.position.y;
            temp_point.z += scanner.position.z;
            total_points.insert(temp_point);
        }
    }
    println!("Total Points: {}", total_points.iter().count());
    let mut large_man = 0;
    for scanner in &scanners {
        for scanner2 in &scanners {
            if scanner != scanner2 {
                let x = scanner.position.x - scanner2.position.x;
                let y = scanner.position.y - scanner2.position.y;
                let z = scanner.position.z - scanner2.position.z;
                let total = x + y + z;
                if total > large_man {
                    large_man = total;
                }
            }
        }
    }
    println!("The Largest Distance: {}", large_man);
}

fn main() {
    star_1(read_a_file("C:/input/input19.txt").unwrap());
}
