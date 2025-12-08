use std::error::Error;

use crate::helpers::read_input;

/// Given a safe with the numbers 0 -> 99 that starts at 50 and a series of instructions for turning left
///  and right an amount, find the secret password to the north pole (the number of times the safe points
///  to 0 after an instruction).
fn find_safe_password(instructions: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut curr_safe_pos = 50;
    let mut password = 0;

    for instruction in instructions {
        let (dir, offset_str) = instruction.split_at(1);
        let offset: i32 = offset_str.parse()?;

        match dir {
            "L" => {
                curr_safe_pos -= offset;
                if curr_safe_pos < 0 {
                    curr_safe_pos += 100;
                }
            }
            "R" => {
                curr_safe_pos += offset;
                if curr_safe_pos > 99 {
                    curr_safe_pos -= 100;
                }
            }
            _ => return Err(format!("Unrecognized direction string, {}", dir).into())
        }

        curr_safe_pos %= 100;

        if curr_safe_pos == 0 {
            password += 1;
        }
    }

    Ok(password)
}

#[allow(unused)]
pub fn day1_find_safe_password_p1() {
    let input = read_input::read_input("1", "txt")
        .expect("Couldn't find the input at the specified path!")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    let result = find_safe_password(&input);

    match result {
        Ok(v) => println!("Puzzle result: {}", v),
        Err(e) => panic!("Error solving puzzle: {:?}", e)
    }
}

/// In the part 2 to this, we are asked to modify the function so that we count the password any time a
///  click crosses 0.
fn find_safe_password_p2(instructions: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut curr_safe_pos = 50;
    let mut password = 0;

    for instruction in instructions {
        let (dir, offset_str) = instruction.split_at(1);
        let offset: i32 = offset_str.parse()?;

        password += offset / 100;

        match dir {
            "L" => {
                let mut new_pos = curr_safe_pos - (offset % 100);
                
                if new_pos < 0 {
                    new_pos += 100;
                }

                let crossed_zero = curr_safe_pos != 0 && curr_safe_pos < new_pos;

                if new_pos == 0 || crossed_zero {
                    password += 1;
                }

                curr_safe_pos = new_pos;
            }
            "R" => {
                let mut new_pos = curr_safe_pos + (offset % 100);

                if new_pos >= 100 {
                    new_pos -= 100;
                }

                let crossed_zero = curr_safe_pos > new_pos;

                if crossed_zero {
                    password += 1;
                }
                
                curr_safe_pos = new_pos;
            }
            _ => return Err(format!("Unrecognized direction string, {}", dir).into())
        }

        println!("offset {} left/right {}. new val: {}, crossed 0 {} times", offset, dir, curr_safe_pos, password);
    }

    Ok(password)
}

#[allow(unused)]
pub fn day1_find_safe_password_p2() {
    let input = read_input::read_input("1", "txt")
        .expect("Couldn't find the input at the specified path!")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    let result = find_safe_password_p2(&input);

    match result {
        Ok(v) => println!("Puzzle result: {}", v),
        Err(e) => panic!("Error solving puzzle: {:?}", e)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let instructions = vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(find_safe_password(&instructions).expect("Input is malformed"), 3);
    }

    #[test]
    fn p2_example() {
        let instructions = vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(find_safe_password_p2(&instructions).expect("Input is malformed"), 6);
    }

    #[test]
    fn p2_handle_large_offsets_2() {
        let instructions = vec![
            "R1000", // +10 (50) 10
            "L1000", // +10 (50) 20
            "L50",   // +1  (0)  21
            "R1",    // +0  (1)  21
            "L1",    // +1  (0)  22
            "L1",    // +0  (99) 22
            "R1",    // +1  (0)  23
            "R100",  // +1  (0)  24
            "R1",    // +0  (1)  24
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(find_safe_password_p2(&instructions).expect("Input is malformed"), 24);
    }
}