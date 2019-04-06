mod diff;

use serde_json::{Value, error::Error};
use std::env;
use std::fs::File;
use std::io::Read;
use std::result::Result;


fn json_in_file(file: &str) -> Value {
    let mut buffer = Vec::new();
    File::open(file).and_then(|mut f| f.read_to_end(&mut buffer)).expect(&format!("Could not read {}", &file));

    serde_json::from_slice(buffer.as_slice()).expect("Could not read JSON from file")
}

fn main() -> Result<(), Error> {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.len() == 2 {
        let left_path = arguments.get(0).unwrap();
        let right_path = arguments.get(1).unwrap();

        let left_value = json_in_file(left_path);
        let right_value = json_in_file(right_path);

        let differences = crate::diff::calculate(left_value, right_value);

        println!("{:#?}", differences);
    }

    Ok(())
}
