use serde_json::Value;

mod diff;

fn main() -> std::result::Result<(), serde_json::error::Error> {
    // Some JSON input left_data as a &str. Maybe this comes from the user.
    let left_data = r#"
    {
		"Aidan Gillen": {
			"array": ["Game of Thron\"es", "The Wire"],
			"string": "some string",
			"int": 2,
			"aboolean": true,
			"boolean": true,
			"object": {
				"foo": "bar",
				"object1": { "new prop1": "new prop value" },
				"object2": { "new prop1": "new prop value" },
				"object3": { "new prop1": "new prop value" },
				"object4": { "new prop1": "new prop value" }
			}
		},
		"Amy Ryan": { "one": "In Treatment", "two": "The Wire" },
		"Annie Fitzgerald": ["Big Love", "True Blood"],
		"Anwan Glover": ["Treme", "The Wire"],
		"Alexander Skarsgard": ["Generation Kill", "True Blood"],
		"Clarke Peters": null
	}
        "#;

    let right_data = r#"
    {
		"Aidan Gillen": {
			"array": ["Game of Thrones", "The Wire"],
			"string": "some string",
			"int": "2",
			"otherint": 4,
			"aboolean": "true",
			"boolean": false,
			"object": {
				"foo": "bar"
			}
		},
		"Amy Ryan": ["In Treatment", "The Wire"],
		"Annie Fitzgerald": ["True Blood", "Big Love", "The Sopranos", "Oz"],
		"Anwan Glover": ["Treme", "The Wire"],
		"Alexander Skarsg?rd": ["Generation Kill", "True Blood"],
		"Alice Farmer": ["The Corner", "Oz", "The Wire"]
	}
    "#;

    let left_value: Value = serde_json::from_str(left_data)?;
    let right_value: Value = serde_json::from_str(right_data)?;


    let differences = crate::diff::calculate(left_value, right_value);

    println!("{:#?}", differences);

    Ok(())
}
