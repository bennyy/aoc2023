use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn file_to_string(file_name: String) -> String {
    let path = Path::new(&file_name);
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_data_string = String::new();
    if let Err(why) = file.read_to_string(&mut file_data_string) {
        panic!("couldn't read {}: {}", path.display(), why)
    }
    file_data_string
}

#[allow(dead_code)]
pub fn convert_string_of_ints_to_list(input_str: String) -> Vec<i32> {
    // Convert list of strings into a u32 vector.
    input_str
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn index_to_coord(index: usize, width: usize) -> (usize, usize) {
    let x = index % width;
    let y = index / width;
    (x, y)
}

#[allow(dead_code)]
fn coord_to_index(coord: (usize, usize), width: usize) -> usize {
    coord.0 + width * coord.1
}
