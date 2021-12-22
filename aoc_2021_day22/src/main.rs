use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
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
    let mut on_points = HashSet::new();
    let mut instructions = Vec::new();
    for line in input {
        let (instruction, box_coord) = line.split_once(" ").unwrap();
        let box_coord = box_coord
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "");
        let splits = box_coord.split(",").collect::<Vec<&str>>();
        let x_range = splits[0];
        let y_range = splits[1];
        let z_range = splits[2];
        let (x_min, x_max) = x_range.split_once("..").unwrap();
        let (y_min, y_max) = y_range.split_once("..").unwrap();
        let (z_min, z_max) = z_range.split_once("..").unwrap();
        instructions.push((
            if instruction == "on" { 1 } else { 0 },
            x_min.parse::<i64>().unwrap(),
            x_max.parse::<i64>().unwrap(),
            y_min.parse::<i64>().unwrap(),
            y_max.parse::<i64>().unwrap(),
            z_min.parse::<i64>().unwrap(),
            z_max.parse::<i64>().unwrap(),
        ));
    }
    for inst in instructions {
        if inst.2 < -50 || inst.4 < -50 || inst.6 < -50 || inst.1 > 50 || inst.3 > 50 || inst.5 > 50
        {
            continue;
        }
        for x in inst.1..inst.2 + 1 {
            for y in inst.3..inst.4 + 1 {
                for z in inst.5..inst.6 + 1 {
                    if inst.0 == 1 {
                        on_points.insert((x, y, z));
                    } else {
                        on_points.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("{}", on_points.iter().count());
}

fn star_2(file: &str) {
    let input = read_a_file(file).unwrap();
    let mut instructions = Vec::new();
    for line in input {
        let (instruction, box_coord) = line.split_once(" ").unwrap();
        let box_coord = box_coord
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "");
        let splits = box_coord.split(",").collect::<Vec<&str>>();
        let x_range = splits[0];
        let y_range = splits[1];
        let z_range = splits[2];
        let (x_min, x_max) = x_range.split_once("..").unwrap();
        let (y_min, y_max) = y_range.split_once("..").unwrap();
        let (z_min, z_max) = z_range.split_once("..").unwrap();
        instructions.push((
            if instruction == "on" { 1 } else { 0 },
            x_min.parse::<i64>().unwrap(),
            x_max.parse::<i64>().unwrap(),
            y_min.parse::<i64>().unwrap(),
            y_max.parse::<i64>().unwrap(),
            z_min.parse::<i64>().unwrap(),
            z_max.parse::<i64>().unwrap(),
        ));
    }
    let mut on_regions = HashSet::new();
    for inst in instructions {
        overlap(&inst, &mut on_regions);
    }
    let mut total = 0;
    for cube in on_regions {
        total += (cube.2 - cube.1 + 1) * (cube.4 - cube.3 + 1) * (cube.6 - cube.5 + 1);
    }
    println!("{}", total);
}

fn overlap(
    inst: &(i32, i64, i64, i64, i64, i64, i64),
    on_regions: &mut HashSet<(i32, i64, i64, i64, i64, i64, i64)>,
) {
    let (ix_min, ix_max, iy_min, iy_max, iz_min, iz_max) =
        (inst.1, inst.2, inst.3, inst.4, inst.5, inst.6);
    let mut add_cubes = Vec::new();
    let mut remove_cubes = Vec::new();
    for onst in on_regions.clone().iter() {
        let (mut ox_min, mut ox_max, mut oy_min, mut oy_max, oz_min, oz_max) =
            (onst.1, onst.2, onst.3, onst.4, onst.5, onst.6);
        //if we don't overlap at all, continue
        if ix_min > ox_max
            || iy_min > oy_max
            || iz_min > oz_max
            || ix_max < ox_min
            || iy_max < oy_min
            || iz_max < oz_min
        {
            continue;
        }
        //if the regions _do_ overlap, boolean out the chunk and convert the existing region to
        //smaller prisms, make a list of cubes to remove, and a list of cubes to add
        //then process it all after the for loop
        remove_cubes.push(onst.clone());
        //setup the overlapping area between the prisms on each axis
        let mx_min = max(ix_min, ox_min);
        let mx_max = min(ix_max, ox_max);
        let my_min = max(iy_min, oy_min);
        let my_max = min(iy_max, oy_max);
        let mz_min = max(iz_min, oz_min);
        let mz_max = min(iz_max, oz_max);
        //create up to 6 new prisms depending on how the iterated prism falls within the bounds
        //calculated above. If splitting a cube, update the temp values for the iterated prism
        //such that nothing is double counted.
        if ox_min < mx_min {
            add_cubes.push((0, ox_min, mx_min - 1, oy_min, oy_max, oz_min, oz_max));
            ox_min = mx_min;
        }
        if ox_max > mx_max {
            add_cubes.push((0, mx_max + 1, ox_max, oy_min, oy_max, oz_min, oz_max));
            ox_max = mx_max;
        }
        if oy_min < my_min {
            add_cubes.push((0, ox_min, ox_max, oy_min, my_min - 1, oz_min, oz_max));
            oy_min = my_min;
        }
        if oy_max > my_max {
            add_cubes.push((0, ox_min, ox_max, my_max + 1, oy_max, oz_min, oz_max));
            oy_max = my_max;
        }
        if oz_min < mz_min {
            add_cubes.push((0, ox_min, ox_max, oy_min, oy_max, oz_min, mz_min - 1));
        }
        if oz_max > mz_max {
            add_cubes.push((0, ox_min, ox_max, oy_min, oy_max, mz_max + 1, oz_max));
        }
    }
    for cube in remove_cubes {
        on_regions.remove(&cube);
    }
    for cube in add_cubes {
        on_regions.insert(cube);
    }
    //if inst.0 = 1, add the region in full
    if inst.0 == 1 {
        on_regions.insert(inst.clone());
    }
}

fn main() {
    star_1("C:/input/input22.txt");
    star_2("C:/input/input22.txt");
}
