use std::env::current_dir;
use std::path::Path;
use std::error::Error;

/// Read an input file based on the day and part of the project.
/// 
/// This function returns a vector of Strings representing the inputs in the input file
/// separated by newlines.
/// 
/// ## Parameters
/// id is the day of the puzzle you want to read the input for as a string slice.
/// Example: "1" or "2".
pub fn read_input(id: &str, file_ext: &str) -> Result<String, Box<dyn Error>> {
    let curr_dir = current_dir()
        .expect("Home should exist");

    let path_string = format!("{}/../days/{}/input.{}", curr_dir.display(), id, file_ext);
    let input_path = Path::new(&path_string);

    if let Ok(false) = input_path.try_exists() {
        return Err(format!("Path {} does not exist. Got transformed to {}", path_string, input_path.display()).into());
    }

    let contents = std::fs::read_to_string(input_path)?;

    Ok(contents)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_day_1_input() {
        let read_result = read_input("1", "txt");

        match read_result {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{:?}", e);
                panic!()
            }
        }
    }
}