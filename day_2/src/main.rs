//use std::io::{self, Read};

fn run_ops(seq: &[u32]) -> Result<Vec<u32>, String> {
    let total = seq.len();
    let mut new = seq.to_vec();
    let mut cur = 0;
    loop {
        if cur >= (total - 4) {
            return Ok(new);
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
    input[1] = 12;
    input[2] = 2;

    let small = [1,0,0,0,99];
    let med = [1,1,1,4,99,5,6,0,99];
    return match run_ops(&input) { // => 3895705
    //return match run_ops(&small) { => 2
    //return match run_ops(&med) { => 30
        Ok(res) => println!("{}", res[0]),
        Err(msg) => println!("Error {}", msg)
    };
}
