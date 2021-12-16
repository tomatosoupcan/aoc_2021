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
#[derive(Debug)]
struct TrData {
    ver: u8,           //the version id
    id: u8,            //the type id
    subs: Vec<TrData>, //a vector of trs contained by this tr
    lt_id: u8,         //the lt id, where applicable
    literal: i64,      //the literal value, where applicable
}

fn hex_2_binary(hex: &Vec<char>) -> Vec<u8> {
    let mut output = Vec::new();
    let i = 1 as u8; //this saves time
    let o = 0 as u8; //having to type u8 over and over
    for chr in hex {
        output.extend(match chr {
            '0' => vec![o, o, o, o],
            '1' => vec![o, o, o, i],
            '2' => vec![o, o, i, o],
            '3' => vec![o, o, i, i],
            '4' => vec![o, i, o, o],
            '5' => vec![o, i, o, i],
            '6' => vec![o, i, i, o],
            '7' => vec![o, i, i, i],
            '8' => vec![i, o, o, o],
            '9' => vec![i, o, o, i],
            'A' => vec![i, o, i, o],
            'B' => vec![i, o, i, i],
            'C' => vec![i, i, o, o],
            'D' => vec![i, i, o, i],
            'E' => vec![i, i, i, o],
            'F' => vec![i, i, i, i],
            _ => continue,
        })
    }
    output
}

fn binary_2_struct(bin: &mut Vec<u8>, total: &mut i32) -> TrData {
    let mut subs = Vec::new();
    let mut ver = 0;
    //separate off the version number
    for _ in 0..3 {
        ver <<= 1;
        ver ^= bin.remove(0);
    }
    *total += ver as i32;
    let mut id = 0;
    //separate off the id
    for _ in 0..3 {
        id <<= 1;
        id ^= bin.remove(0);
    }
    let mut lt_id = 0;
    //if not id 4, we need to get the lt_id, if 4 store a placeholder for lt_id
    if id != 4 {
        lt_id <<= 1;
        lt_id ^= bin.remove(0);
    } else {
        lt_id = u8::MAX;
    }
    let mut literal: i64 = 0;
    //if lt_id is the max u8 value, that means it is not present for this operation
    //all we need to do is find where the transmission ends, strip the identifiers
    //and concatenate by pushing bits onto the value, then store this as the literal
    if lt_id == u8::MAX {
        let mut finding = true;
        let mut depth = 0;
        while finding {
            if bin[depth] == 0 {
                finding = false;
            } else {
                depth += 5;
            }
        }
        depth += 5;
        for a in 0..depth {
            if a % 5 == 0 {
                bin.remove(0);
            } else {
                literal <<= 1;
                literal ^= bin.remove(0) as i64;
            };
        }
    }
    //if lt_id is 0, we need to parse a specific range as transmissions
    //this is easy enough to do simply by popping off the range and running it
    //through this function recursively
    else if lt_id == 0 {
        let bit_count = 15;
        let mut sp_len = 0;
        for _ in 0..bit_count {
            sp_len <<= 1;
            sp_len ^= bin.remove(0) as i64;
        }
        let mut new_vec = Vec::new();
        for _ in 0..sp_len {
            new_vec.push(bin.remove(0));
        }
        while new_vec.iter().map(|x| *x as i32).sum::<i32>() != 0 {
            subs.push(binary_2_struct(&mut new_vec, total));
        }
    }
    //if lt_id is 1, we need to split it into the proper number of transmissions
    //we do this by simply passing the binary vector a number of times equal to
    //the number of transmissions identified in the 11 bit identifier
    //since they will already terminate themselves accordingly each time
    else if lt_id == 1 {
        let bit_count = 11;
        let mut sp_count = 0;
        for _ in 0..bit_count {
            sp_count <<= 1;
            sp_count ^= bin.remove(0) as i32;
        }
        for _ in 0..sp_count {
            subs.push(binary_2_struct(bin, total));
        }
    }
    return TrData {
        ver,
        id,
        subs,
        lt_id,
        literal,
    };
}

fn pro(structure: &TrData) -> i64 {
    let subs_s = &structure.subs;
    let subs = subs_s.iter().map(|x| pro(x));
    return match &structure.id {
        0 => subs.fold(0, |acc, x| x + acc),              //0 is addition
        1 => subs.fold(1, |acc, x| x * acc),              //1 is multiplication
        2 => subs.min().unwrap(),                         //2 is minimum value
        3 => subs.max().unwrap(),                         //3 is max value
        5 => (pro(&subs_s[0]) > pro(&subs_s[1])) as i64,  //5 is greater than
        6 => (pro(&subs_s[0]) < pro(&subs_s[1])) as i64,  //6 is less than
        7 => (pro(&subs_s[0]) == pro(&subs_s[1])) as i64, //7 is equal to
        _ => structure.literal,                           //all else return the value
    };
}

fn star_1_2() {
    let input = read_a_file("C:/input/input16.txt").unwrap();
    let hex_chars = input[0].chars().collect::<Vec<char>>();
    let mut binary_convert = hex_2_binary(&hex_chars);
    let mut ver_total = 0;
    let data_struct = binary_2_struct(&mut binary_convert, &mut ver_total);
    println!(
        "Final Struct: {:?} \nFinal Bits: {:?}",
        data_struct, binary_convert
    );
    println!("Version Sum: {}", ver_total);
    println!("Processed Value: {}", pro(&data_struct));
}

fn main() {
    star_1_2();
}
