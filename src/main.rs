use serde_json::{Result, Value};

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
    let v: Value = serde_json::from_str(left_data)?;

    println!("{:#?}", v);

    // Access parts of the left_data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
