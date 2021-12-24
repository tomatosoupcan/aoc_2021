fn star_1_2() {
    let mut allowed = Vec::new();
    for w2 in 1..10 {
        let w3 = w2 - 1;
        for w4 in 1..10 {
            let w5 = w4 + 5;
            for w6 in 1..10 {
                let w7 = w6 + 8;
                for w1 in 1..10 {
                    let w8 = w1 + 4;
                    for w0 in 1..10 {
                        let w9 = w0 + 3;
                        for w11 in 1..10 {
                            let w12 = w11 - 6;
                            for w10 in 1..10 {
                                let w13 = w10 + 2;
                                let stringed = format!(
                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                    w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13
                                );
                                if stringed.len() == 14 && !stringed.contains('0') {
                                    allowed.push(stringed);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    allowed.sort();
    println!("{:?}", allowed[0]);
    allowed.reverse();
    println!("{:?}", allowed[0]);
}

fn main() {
    /*
    W3 = W2 - 1
    W5 = W4 + 5
    W7 = W6 + 8
    W8 = W1 + 4
    W9 = W0 + 3
    W12 = W11 - 6
    W13 = W10 + 2

    Decompiled the code provided such that I could parse what was happening
    Got the reddit push that Z is a base 26 stack and I was off to the races
     */
    println!("Parsed this by hand again");
    star_1_2();
}
