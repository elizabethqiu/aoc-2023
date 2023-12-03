// use aoc23::*;

// run: `cargo run --bin 2`

fn main() {
    let input = include_str!("2.txt");
    let mut res = 0;
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for line in input.lines() {
        let (id, info) = line.split_once(":").unwrap();
        let rounds: Vec<&str> = info.split(";").into_iter().collect();

        for round in rounds {
            for draw in round.split(",") {
                // ["x green"]
                let stmt: Vec<&str> = draw.split(" ").collect();

                let val = stmt[1].parse::<i32>().unwrap();
                let color = stmt[2];

                if color == "green" {
                    green = green.max(val);
                } else if color == "red" {
                    red = red.max(val);
                } else {
                    blue = blue.max(val);
                }

                // println!("val is {}, color is {}", val, color);
                // println!("Exiting stmt");
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            // println!("adding id {} to res {}", id, res);
            res += id.replace("Game ", "").to_string().parse::<i32>().unwrap(); // replace &str portion, convert to String, convert to int, add to res as a valid game
            // println!("now res is {}", res);
        }

        // debugging
        // println!("res is {}, rounds is {:?}", res, rounds);
        blue = 0;
        red = 0;
        green = 0;
    }

    println!("d2p1: {}", res);
}

// brute force / initial ideas:

// 🌙 PART 2
// 0) parsing words....
// 1) issues like "oneight" -> should be "one" as first, "eight" as last = 18.
// 2) should i focus on processing text only, or do text and num together while parsing thru?
// 3) finding first is straightforward, finding last digit will be reversing line, and adding 1 char at a time

//     mapping = {"red": 0, "blue": 0, "green": 0}
//     for set in sets:
//         set = set.strip()
//         if set == "":
//             continue
//         balls = set.split(",")
//         for ball in balls:
//             num_balls, color = ball.strip().split(" ")
//             num_balls = int(num_balls)
//             mapping[color] = max(mapping[color], num_balls)

//     sum_of_games += mapping["red"] * mapping["blue"] * mapping["green"]

// 🌙 PART 1
// observe each line structure, where n varies.
// "description: round; round; round" where each round has csv's
// "Game [X = line num]: n blue, n red; n red, n green, n blue; n green"
// order: blue, red, green (optional to have the all, but each game round must have at LEAST 1)

// split line by delimiter ":" -> unwrap arr of 2 elts
// use arr[1] to get rounds (csv's) separated by ";"

// split round by delimiter ";" -> arr of 1-3 elts
// ["n blue, n red", "n red, n green, n blue", "n green"]
// so much splitting
// and then like day 1, i presume we go in and try to identify these numbers and compare

// preview an iterator: use .collect::<Vec<_>>() and then print the vector
// let first_backward = digits.last().copied().unwrap();
// iterator trait: https://doc.rust-lang.org/std/iter/trait.Iterator.html
// .last()  returns Option<&i32>
// .copied() removes the reference inside
// .unwrap() removes the Option to access the inner i32
