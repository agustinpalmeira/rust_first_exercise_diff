use std::fs::File;
use std::io::Read;

/// Reads a text file and convert it to an string vector, which is the return value.
/// In case the file cannot be read, an empty vector is returned.
///
/// # Example
///
/// Basic usage:
/// ```
/// let file: &Vec<String> = &file_reader::read_file_lines("file_path");
/// ```
pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let mut text_vector: Vec<String> = Vec::new();

    return match File::open(file_path) {
        Ok(mut file) => {
            let mut file_text = String::new();
            file.read_to_string(&mut file_text).unwrap();

            let line_separator = "\n";
            let split = file_text.split(line_separator);

            for s in split {
                text_vector.push(s.to_string());
            }

            text_vector
        }
        Err(error) => {
            println!("Cannot open the file {}: {}", file_path, error);
            text_vector
        }
    };
}
