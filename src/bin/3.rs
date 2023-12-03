// use aoc23::*;

// run: `cargo run --bin n`

fn main() {
    let input = include_str!("3.txt");

    let res = 0;
    let res2 = 0;

    // todo

    println!("d3p1: {}", res);
    println!("d3p2: {}", res2);
}

// brute force / initial ideas:

// 🌙 PART 2

// 🌙 PART 1
// looks like BFS in rust
// brute force going to look at borders of every number and if there's something not a (.), then add to res
// 1) how do we identify a number? certainly we don't want to start looking at the border each time we see a number, otherwise we'll always add it to the result. in a case like
// .....
// .345.
// .....
// it is invalid and should not be added. but if we see "3" and start searching
// the border, we'll run into "4", which makes it valid
// 2) should we implement the entire file as a 2d vector
// 3) or maybe assign coordinates to everything?

// grid in rust
// defaultmap

// .replace("strtoreplace", "replacement")
// .to_string() for &str -> String
// .parse::<i32>() for getting int from String
// .unwrap() removes the Option to access the inner i32
