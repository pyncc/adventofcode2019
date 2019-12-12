use std::io;
use std::i32::MAX;
use std::collections::HashSet;

fn get_wire_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");

    return line;
}

fn make_wire((x, y): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut cur_x = x;
    let mut cur_y = y;
    let mut wire: HashSet<(i32, i32)> = HashSet::new();
    for item in get_wire_input().split(',') {
        let (dir, _count) = item.trim().split_at(1);
        let count = _count.parse::<u32>().expect("Failed to parse step count");

        match dir {
            "R" => {
                // go +count in x direction
                let target = cur_x + count as i32;
                while cur_x <= target {
                    cur_x += 1;
                    wire.insert((cur_x, cur_y));
                    //println!("{}: {} <= {}", count, cur_x, (cur_x + count as i32));
                };
            },
            "U" => {
                // go +count in y direction
                let target = cur_y + count as i32;
                while cur_y <= target {
                    cur_y += 1;
                    wire.insert((cur_x, cur_y));
                };
            },
            "D" => {
                // go -count in y direction
                let target = cur_y - count as i32;
                while cur_y >= target {
                    cur_y -= 1;
                    wire.insert((cur_x, cur_y));
                };
            },
            "L" => {
                // go -count in x direction
                let target = cur_x - count as i32;
                while cur_x >= target {
                    cur_x -= 1;
                    wire.insert((cur_x, cur_y));
                };
            },
            _ => {
                panic!("Unknown direction {}", dir);
            }
        }
    }

    wire
}

fn main() {
    // 2 wires, described in input as stateful directions (eg R10, U2, D10) tracing the wires path in 2D space
    // origin is same for all wires, is implicitly defined as 0,0
    // 1. Find intersections between the wires
    //    wires can cross, but crosses with themselves do not count
    //    This gives a nice simplification since it can be modeled as
    //    N sets of coordinates when is amenable to seet operations
    //    to find intersections (wire set coordinate overlaps)
    // 2. Find the intersection with the minimum distance to origin,
    //    using the sum of the absolute difference in their cartisian
    //    coordinates. Since its relative to origin, this is just the
    //    intersection with the minimum sum.
    //
    // 0. Setup
    //   - input is: one line per wire, comma-separated directions
    //   - per line:
    //     * initialize a set of i32 2-tuples
    //     * initialize starting coordinate (0,0)
    //   - per comma-separated item:
    //     * parse the item into [direction, count]
    //     * using the direction, add/subtract x or y for count places from the current coordinate, pushing each of these into the set
    // 1. Find intersections
    //   - perform a set intersection between the two sets
    // 2. Find the closest to origin
    //   - return the minimum from this intersection based on the sum of each tuple
    let first = make_wire((0,0));
    println!("first wire is this long: {}", first.len());
    let second = make_wire((0,0));
    println!("second wire is this long: {}", second.len());

    let mut cur_min = std::i32::MAX;
    for (x, y) in first.intersection(&second) {
        let distance = x.abs() + y.abs();
        if cur_min > distance {
            cur_min = distance;
        }
    }

    println!("min distance: {}", cur_min);
}
