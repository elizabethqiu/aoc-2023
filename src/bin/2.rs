// use aoc23::*;

// run: `cargo run --bin 2`

fn main() {
    let input = include_str!("2.txt");
    let mut res = 0;
    let mut res2 = 0;
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
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            res += id.replace("Game ", "").to_string().parse::<i32>().unwrap(); // replace &str portion, convert to String, convert to int, add to res as a valid game
        }

        let powerset = red * green * blue;
        println!("adding powerset {} to res2 {}", powerset, res2);
        res2 += powerset;

        // debugging
        blue = 0;
        red = 0;
        green = 0;
    }

    println!("d2p1: {}", res);
    println!("d2p2: {}", res2);
}

// brute force / initial ideas:

// 🌙 PART 2
// powerset = multiply all max vals of red, green, blue per line
// add to total (res2)

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

// .replace("strtoreplace", "replacement")
// .to_string() for &str -> String
// .parse::<i32>() for getting int from String
// .unwrap() removes the Option to access the inner i32
