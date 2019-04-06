use serde_json::{Value, Map};

fn stringify(val: Value) -> String {
    let mut output = String::new();
    print(&mut output, val);

    output
}

fn print(output: &mut String, value: Value) {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(b) => output.push_str(&b.to_string()),
        Value::Number(n) => output.push_str(&n.to_string()),
        Value::String(n) => string(output, &n),
        Value::Array(values) => print_array(output, values),
        Value::Object(object) => print_object(output, object)
    }
}

fn print_object(output: &mut String, object: Map<String, Value>) {
    output.push_str("{");
    for key in object.keys() {
        string(output, key);
        output.push_str(":");
        print(output, object.get(key).unwrap().clone())
    }
    output.push_str("}");
}

fn print_array(output: &mut String, values: Vec<Value>) {
    output.push_str("[");
    let number_of_items = values.len();
    let mut current = 0;

    for value in values {
        print(output, value);

        if current != number_of_items - 1 {
            output.push_str(", ");
        }
        current += 1;

    }
    output.push_str("]");
}

fn string(output: &mut String, value: &str) {
    output.push_str(&format!("\"{}\"", value));
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use serde_json::json;

    #[test]
    fn prints_an_object() {
        assert_eq!("{}", stringify(json!({})));
    }

    #[test]
    fn prints_null() {
        assert_eq!("null", stringify(json!(null)));
    }

    #[test]
    fn prints_a_string() {
        assert_eq!("\"foo\"", stringify(json!("foo")));
    }

    #[test]
    fn prints_object_with_key_value_pair() {
        assert_eq!("{\"a\":\"foo\"}", stringify(json!({"a": "foo"})));
    }

    #[test]
    fn prints_a_boolean() {
        assert_eq!("true", stringify(json!(true)));
    }

    #[test]
    fn prints_a_number() {
        assert_eq!("4.67", stringify(json!(4.67)));
    }

    #[test]
    fn prints_an_array() {
        assert_eq!("[1, 2, \"foo\"]", stringify(json!([1,2,"foo"])));
    }

    #[test]
    fn complex_object() {
        assert_eq!("{\"foo\":{\"a\":[1, 2, 3]\"b\":{\"c\":true}}}", stringify(json!({"foo": {"a": [1,2,3], "b": {"c": true}}})))
    }
}
