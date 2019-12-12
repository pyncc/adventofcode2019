use std::io::{self, BufRead};

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
    //     * initialize a set of u32 2-tuples
    //     * initialize starting coordinate (0,0)
    //   - per comma-separated item:
    //     * parse the item into [direction, count]
    //     * using the direction, add/subtract x or y for count places from the current coordinate, pushing each of these into the set
    // 1. Find intersections
    //   - perform a set intersection between the two sets
    // 2. Find the closest to origin
    //   - return the minimum from this intersection based on the sum of each tuple
    for line in io::stdin().lock().lines() {
        println!("{}", line.unwrap());
    }
}
