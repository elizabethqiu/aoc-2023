// use aoc23::*;

fn main() {
    let input = include_str!("1.txt");
    let mut res = 0;
    let mut first_forward: u32 = 0;
    let mut first_backward: u32 = 0;
    let nums = [
        "placeholder",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    for line in input.lines() {
        // println!("{}", line);

        let mut line_vec: Vec<char> = line.chars().collect(); // returns an iterator, and I convert it to a vector

        // find first_forward
        'outer_forward: for i in 0..line_vec.len() {
            // DIGITS (d1p1)
            // https://doc.rust-lang.org/std/primitive.char.html
            if line_vec[i].is_numeric() {
                // first_forward = line_vec[i].to_digit(10).unwrap();
                if let Some(digit) = line_vec[i].to_digit(10) {
                    first_forward = digit;
                    break;
                }
            }

            // ACTUAL NUM
            for n in 1..nums.len() {
                let nums_char_slice: Vec<char> = nums[n].chars().collect();
                if line_vec[i..].starts_with(&nums_char_slice[..]) {
                    first_forward = n as u32;
                    break 'outer_forward;
                }
            }
        }

        // find first_backward
        line_vec.reverse();
        let mut temp = String::new();
        // let mut temp: Vec<char> = Vec::new();
        // line_vec.iter().rev();

        // so quirky this is different from finding first_forward
        'outer_backward: for ch in line_vec.iter() {
            if ch.is_numeric() {
                first_backward = ch.to_digit(10).unwrap();
                break;
            }

            // ACTUAL NUM
            // this is tricky because we need to add a letter to the front of the vector each time // update: just use String. prepend to String using format
            // temp.insert(0, line_vec[i]);
            temp = format!("{}{}", ch, temp);
            for n in 1..nums.len() {
                if temp.starts_with(nums[n]) {
                    first_backward = n as u32;
                    break 'outer_backward;
                }
            }
        }

        println!("ff is {}, fb is {}", first_forward, first_backward);
        res += first_forward * 10 + first_backward;

        first_forward = 0;
        first_backward = 0;
    }

    println!("d1p1 + 2: {}", res);
}

// brute force / initial ideas:

// 🌙 PART 2
// 0) parsing words....
// 1) issues like "oneight" -> should be "one" as first, "eight" as last = 18.
// 2) should i focus on processing text only, or do text and num together while parsing thru?
// 3) finding first is straightforward, finding last digit will be reversing line, and adding 1 char at a time

// 🌙 PART 1
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
