use std::error::Error;

use crate::helpers::read_input::read_input;

#[allow(unused)]
pub fn day3_find_total_maximum_joltage_p1() {
    let input = read_input("3", "txt")
        .expect("Couldn't find the input at the specified path!")
        .split_terminator("\n")
        .map(|s| s.to_string())
        .collect();

    let result = find_total_max_joltage(input);

    match result {
        Ok(v) => println!("Puzzle result: {}", v),
        Err(e) => panic!("Error solving puzzle: {:?}", e)
    }
}

#[allow(unused)]
pub fn day3_find_total_maximum_joltage_p2() {
    // let input = read_input("2", "txt")
    //     .expect("Couldn't find the input at the specified path!")
    //     .split(",")
    //     .map(|s| s.to_string())
    //     .collect();

    // let result = p2_find_invalid_ids(&input);

    // match result {
    //     Ok(v) => println!("Puzzle result: {}", v),
    //     Err(e) => panic!("Error solving puzzle: {:?}", e)
    // }
}

/// Find maximum joltage of a battery bank by finding the two largest unique values in the bank.
/// Time complexity O(2n) should be possible for the inner algorithm
fn find_total_max_joltage(banks: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let fold_res = banks
    .iter()
    .fold(Ok(String::from("0")), |acc: Result<String, Box<dyn Error>>, bank: &String| {
        let digit_vec: Vec<i32> = bank
            .chars()
            .map(|c| c.to_string().parse().expect("char should be a valid digit")).collect();

        let mut max_jolt = 0;

        for first in 0..digit_vec.len() {
            for second in first + 1..digit_vec.len() {
                let potential = digit_vec[first] * 10 + digit_vec[second];

                if potential > max_jolt {
                    max_jolt = potential;
                }
            }
        }
        
        let res = acc.unwrap_or("0".to_string()).parse::<i32>().expect("should be valid digit") + max_jolt;
        
        Ok(res.to_string())
    });

    match fold_res {
        Ok(v) => Ok(v.to_string().parse().expect("should be valid i32")),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let input  = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111"  
        ]
        .iter().map(|s| s.to_string().parse().expect("should be able to parse string into u64"))
        .collect();

        assert_eq!(find_total_max_joltage(input).expect("error occured"), 357)
    }
}