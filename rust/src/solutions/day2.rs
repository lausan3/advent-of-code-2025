use std::{error::Error};
use crate::helpers::read_input::read_input;

#[allow(unused)]
pub fn day2_find_invalid_ids_p1() {
    let input = read_input("2", "txt")
        .expect("Couldn't find the input at the specified path!")
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let result = find_invalid_ids(&input);

    match result {
        Ok(v) => println!("Puzzle result: {}", v),
        Err(e) => panic!("Error solving puzzle: {:?}", e)
    }
}

#[allow(unused)]
pub fn day2_find_invalid_ids_p2() {
    let input = read_input("2", "txt")
        .expect("Couldn't find the input at the specified path!")
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let result = p2_find_invalid_ids(&input);

    match result {
        Ok(v) => println!("Puzzle result: {}", v),
        Err(e) => panic!("Error solving puzzle: {:?}", e)
    }
}

fn find_invalid_ids(id_ranges: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let mut sum_of_invalid_ids = 0;

    for range in id_ranges {
        let (start, end): (i64, i64) = range
            .split_once("-")
            .map(|(s, e)| {
                (s.trim().parse().expect(format!("failed to parse start of range {}. {}", range, s).as_str()), 
                 e.trim().parse().expect(format!("failed to parse end of range {}. {}", range, e).as_str())) 
            }
            )
            .ok_or("Range split error")?;

        for num in start..=end {
            if !is_id_valid(num.to_string()) {
                sum_of_invalid_ids += num;
                println!("{} was invalid", num);
            }
        }
    }

    Ok(sum_of_invalid_ids)
}

fn is_id_valid(id: String) -> bool {
    let id_size = id.len();

    let odd = id_size % 2 == 1;
    let unequal_halves = id[..id_size / 2] != id[id_size / 2..];

    odd || unequal_halves
}

fn p2_find_invalid_ids(id_ranges: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let mut sum_of_invalid_ids = 0;

    for range in id_ranges {
        let (start, end): (i64, i64) = range
            .split_once("-")
            .map(|(s, e)| {
                (s.trim().parse().expect(format!("failed to parse start of range {}. {}", range, s).as_str()), 
                 e.trim().parse().expect(format!("failed to parse end of range {}. {}", range, e).as_str())) 
            }
            )
            .ok_or("Range split error")?;

        for num in start..=end {
            if !p2_is_id_valid(num.to_string()) {
                sum_of_invalid_ids += num;
                println!("{} was invalid", num);
            }
        }
    }

    Ok(sum_of_invalid_ids)
}

// This algorithm is O(n)
fn p2_is_id_valid(id: String) -> bool {
    let id_size = id.len();

    for len in 1..=id_size / 2 {
        if id_size % len != 0 {
            continue;
        }
        let pat = &id[..len];
        if pat.repeat(id_size / len) == id {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_invalid() {
        assert_eq!(is_id_valid("123123".into()), false)
    }

    #[test]
    fn test_valid() {
        assert_eq!(is_id_valid("89289".into()), true);
    }

    #[test]
    fn p1_example() {
        let ids_string = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

        let id_ranges = ids_string
            .split_terminator(",")
            .map(|s| s.to_string())
            .collect();

        assert_eq!(find_invalid_ids(&id_ranges).unwrap(), 1227775554);
    }

    #[test]
    fn p2_example() {
        let ids_string = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

        let id_ranges = ids_string
            .split_terminator(",")
            .map(|s| s.to_string())
            .collect();

        assert_eq!(p2_find_invalid_ids(&id_ranges).unwrap(), 4174379265);
    }
}