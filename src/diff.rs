use std::cmp::Ordering;

use serde_json::{Map, Value, Number};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Difference {
    Changed(SlightMutation),
    Added(Value),
    Removed(Value),
}

#[derive(Debug, PartialEq)]
pub struct SlightMutation {
    original_value: Value,
    modified_value: Value,
}

pub fn calculate(left: Value, right: Value) -> Vec<Difference> {
    match (left, right) {
        (Value::Object(l), Value::Object(r)) => object_difference(l, r),
        (Value::Number(n), Value::Number(m)) => number_difference(n, m),
        (Value::Array(left_vals), Value::Array(right_vals)) => {
            array_difference(left_vals, right_vals)
        }
        (Value::String(a), Value::String(b)) => string_differences(a, b),
        (a, b) => vec!(Difference::Changed(SlightMutation{original_value: a, modified_value: b})),
        _ => Vec::new(),
    }
}

fn type_difference(left: Value, right: Value) -> Vec<Difference> {
    Vec::new()
}

fn string_differences(n: String, m: String) -> Vec<Difference> {
    if n != m {
        vec![Difference::Changed(SlightMutation {
            original_value: Value::String(n),
            modified_value: Value::String(m),
        })]
    } else {
        Vec::new()
    }
}

fn number_difference(n: Number, m: Number) -> Vec<Difference> {
    if n != m {
        vec![Difference::Changed(SlightMutation {
            original_value: Value::Number(n),
            modified_value: Value::Number(m),
        })]
    } else {
        Vec::new()
    }
}


pub fn object_difference(
    mut left: Map<String, Value>,
    mut right: Map<String, Value>,
) -> Vec<Difference> {
    let mut all_keys: HashSet<String> = HashSet::new();

    for k in left.keys() {
        all_keys.insert(k.to_string());
    }

    for k in right.keys() {
        all_keys.insert(k.to_string());
    }

    let mut differences = Vec::new();

    for k in all_keys {
        match (left.remove(&k), right.remove(&k)) {
            (Some(v), Some(w)) => differences.append(&mut calculate(v, w)),
            (Some(v), None) => differences.push(Difference::Removed(object_with(k, v))),
            (None, Some(w)) => differences.push(Difference::Added(object_with(k,w))),
            _ => unreachable!(
                "Looks like a key was unexpectedly neither in the left object nor in the right?"
            ),
        }
    }

    differences
}

fn object_with(key: String, value: Value) -> Value {
    let mut the_object = serde_json::Map::new();
    the_object.insert(key, value);
    Value::Object(the_object)
}

pub fn array_difference(mut left_vals: Vec<Value>, mut right_vals: Vec<Value>) -> Vec<Difference> {
    let mut l_iter = left_vals.into_iter();
    let mut r_iter = right_vals.into_iter();

    let mut differences: Vec<Difference> = Vec::new();
    loop {
        match (l_iter.next(), r_iter.next()) {
            (Some(v), Some(w)) => differences.append(&mut calculate(v, w)),
            (Some(v), None) => differences.push(Difference::Removed(v)),
            (None, Some(w)) => differences.push(Difference::Added(w)),
            (None, None) => break,
        }
    }

    differences
}

#[cfg(test)]
mod tests {
    use serde_json;
    use serde_json::json;

    use super::*;

    #[test]
    fn difference_between_numbers() {
        let left_value: Value = json!(1);
        let right_value: Value = json!(2);

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::Changed(SlightMutation {
                original_value: json!(1),
                modified_value: json!(2),
            })),
            difference
        )
    }

    #[test]
    fn array_with_missing_value() {
        let left_value: Value = json!([1, 2]);
        let right_value: Value = json!([1]);

        let difference = calculate(left_value, right_value);

        assert_eq!(vec!(Difference::Removed(json!(2))), difference)
    }

    #[test]
    fn array_with_surpluss_value() {
        let left_value: Value = json!([1, 2]);
        let right_value: Value = json!([1, 2, 3]);

        let difference = calculate(left_value, right_value);

        assert_eq!(vec!(Difference::Added(json!(3))), difference)
    }

    #[test]
    fn object_with_different_string_values() {
        let left_value: Value = json!({"a": "foo"});
        let right_value: Value = json!({"a": "bar"});

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::Changed(SlightMutation {
                original_value: json!("foo"),
                modified_value: json!("bar"),
            })),
            difference
        )
    }

    #[test]
    fn second_object_has_surplus_elements() {
        let left_value: Value = json!({"a": "foo"});
        let right_value: Value = json!({"a": "foo", "b": "bar"});

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::Added(json!({"b": "bar"}))),
            difference
        )
    }

    #[test]
    fn second_object_has_missing_elements() {
        let left_value: Value = json!({"a": "foo", "b": "bar"});
        let right_value: Value = json!({"a": "foo"});

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::Removed(json!({"b": "bar"}))),
            difference
        )
    }
}
