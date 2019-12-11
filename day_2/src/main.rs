//use std::io::{self, Read};

fn run_ops(seq: &[u32], inputs: (u8, u8)) -> Result<u32, String> {
    let total = seq.len();
    if total <= 0 {
        panic!("invalid input sequence length {}", total);
    }
    let mut new = seq.to_vec();
    new[1] = inputs.0 as u32;
    new[2] = inputs.1 as u32;
    let mut cur = 0;
    loop {
        if cur >= (total - 4) {
            return Ok(new[0]);
        }
        let op_code = new[cur];
        let a = new[cur + 1] as usize;
        let b = new[cur + 2] as usize;
        let dst = new[cur + 3] as usize;
        println!("old {}", new[dst]);
        if op_code == 1 {
            new[dst] = new[a] + new[b];
        } else if op_code == 2 {
            new[dst] = new[a] * new[b];
        } else if op_code == 99 {
            return Ok(new[0]);
        } else {
            panic!("unknown opcode {}", op_code);
        }

        println!("new {}", new[dst]);

        cur += 4;
    }
}

// wasting too much time figuring out parsing, translation
// so use vector literal and revisit another time
fn main() {
    let mut input = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,23,13,27,1,10,27,31,2,31,6,35,1,5,35,39,1,39,10,43,2,9,43,47,1,47,5,51,2,51,9,55,1,13,55,59,1,13,59,63,1,6,63,67,2,13,67,71,1,10,71,75,2,13,75,79,1,5,79,83,2,83,9,87,2,87,13,91,1,91,5,95,2,9,95,99,1,99,5,103,1,2,103,107,1,10,107,0,99,2,14,0,0];
    const output_match: u32 = 19690720;
    let mut x = 0u8;
    'outer: loop {
        if x > 99 {
            break 'outer;
        }
        let mut y = 0u8;
        loop {
            if y > 99 {
                break;
            }
            println!("{} {}", x, y);
            match run_ops(&input, (x, y)) {
                Ok(res) => {
                    println!("{}", res);
                    if res == output_match {
                        println!("ANSWER: {} {}", x, y);
                        break 'outer;
                    }
                },
                Err(msg) => {
                    panic!("Error {}", msg);
                }
            };
            y += 1;
        }
        x += 1;
    }
}
