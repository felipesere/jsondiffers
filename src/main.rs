use serde_json::{Value};

mod diff;

fn main() -> std::result::Result<(), serde_json::error::Error> {
    // Some JSON input left_data as a &str. Maybe this comes from the user.
    let left_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of left_data into serde_json::Value.
    let left_value: Value = serde_json::from_str(left_data)?;
    let right_value: Value = serde_json::from_str(left_data)?;

    println!("{:#?}", left_value);
    println!("Please call {} at the number {}", left_value["name"], left_value["phones"][0]);

    crate::diff::calculate(left_value, right_value);

    // Access parts of the left_data by indexing with square brackets.

    Ok(())
}
