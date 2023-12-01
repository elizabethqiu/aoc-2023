// use aoc23::*;

fn main() {
    let input = include_str!("1.txt");
    let mut res = 0;
    let mut first_forward = 0;
    let mut first_backward = 0;

    for line in input.lines() {
        // println!("{}", line);

        let digits: Vec<char> = line.chars().collect(); // returns an iterator, and I convert it to a vector

        for ch in &digits {
            // https://doc.rust-lang.org/std/primitive.char.html
            if ch.is_numeric() {
                if let Some(digit) = ch.to_digit(10) {
                    first_forward = digit;
                    break;
                }
            }
        }

        // digits.reverse();
        for ch in digits.iter().rev() {
            if ch.is_numeric() {
                if let Some(digit) = ch.to_digit(10) {
                    first_backward = digit;
                    break;
                }
            }
        }

        res += first_forward * 10 + first_backward;
    }

    println!("{}", res);
}

// brute force / initial ideas:

// 0) use bufreader? No.
// 1) if no number -> ? (i assume looking at the dataset we are guaranteed at least 1 num per line)
// 2) what library can we use to tell the difference between num and else? regex?
// 3) or can we read forward and backward per line to find the first one on both ends? two pointers? maybe two pointers is less efficient cuz we read 2x more? but keeping track is easier

// preview an iterator: use .collect::<Vec<_>>() and then print the vector
// let first_backward = digits.last().copied().unwrap();
// iterator trait: https://doc.rust-lang.org/std/iter/trait.Iterator.html
// .last()  returns Option<&i32>
// .copied() removes the reference inside
// .unwrap() removes the Option to access the inner i32

// let mut res = 0;
// for each line in file:
//  let bool first_found = false;
//  let mut first_forward = 0;
//  let mut first_backward = 0;
//  for each char in file:
//      if we have found a num
//         first_forward = assigned
//      break
//  for each char in file reading it backwards:
//      if we have found a num
//         first_forward = assigned
//      break
//   res += first_forward + first_backward
// res
