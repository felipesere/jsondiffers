use std::cmp::Ordering;

use serde_json::{Map, Value};
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
        (Value::Number(m), Value::Number(n)) => {
            if m == n {
                Vec::new()
            } else {
                vec![Difference::Changed(SlightMutation {
                    original_value: Value::Number(m),
                    modified_value: Value::Number(n),
                })]
            }
        }
        (Value::Array(left_vals), Value::Array(right_vals)) => {
            array_difference(left_vals, right_vals)
        }
        (Value::String(a), Value::String(b)) => string_differences(a, b),
        (a, b) => type_difference(a,b),
        _ => Vec::new(),
    }
}

fn compare_numbers(n: &serde_json::Number, m: &serde_json::Number) -> Ordering {
    n.as_f64()
        .unwrap()
        .partial_cmp(&m.as_f64().unwrap())
        .unwrap()
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
            (Some(v), None) => differences.push(Difference::Removed(v)),
            (None, Some(w)) => differences.push(Difference::Added(w)),
            _ => unreachable!(
                "Looks like a key was unexpectedly neither in the left object nor in the right?"
            ),
        }
    }

    differences
}

pub fn array_difference(mut left_vals: Vec<Value>, mut right_vals: Vec<Value>) -> Vec<Difference> {
    left_vals.sort_by(values);
    right_vals.sort_by(values);

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

fn values(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(n), Value::Number(m)) => compare_numbers(n, m),
        (Value::Null, Value::Null) => Ordering::Equal,
        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
        (Value::String(a), Value::String(b)) => a.cmp(b),
        (Value::Array(a), Value::Array(b)) => a.len().cmp(&b.len()), // todo
        (Value::Object(a), Value::Object(b)) => a.len().cmp(&b.len()), // todo
        (_, _) => Ordering::Less,                                    // Todo
    }
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
}
