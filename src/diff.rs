use serde_json::{Map, Value};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Difference {
    Changed(SlightMutation),
    Added(Value),
    Removed(Value),
}

impl Difference {
    fn change(original: Value, modified: Value) -> Difference {
        Difference::Changed(SlightMutation {
            original_value: original,
            modified_value: modified,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct SlightMutation {
    original_value: Value,
    modified_value: Value,
}

pub fn calculate(left: Value, right: Value) -> Vec<Difference> {
    use Value::*;

    match (left, right) {
        (l @ Null, r @ Null)
        | (l @ Bool(_), r @ Bool(_))
        | (l @ Number(_), r @ Number(_))
        | (l @ String(_), r @ String(_)) => primitive_difference(l, r),
        (Object(l), Object(r)) => object_difference(l, r),
        (Array(l), Array(r)) => array_difference(l, r),
        (l, r) => type_difference(l, r),
    }
}

fn type_difference(a: Value, b: Value) -> Vec<Difference> {
    vec![Difference::change(a, b)]
}

fn primitive_difference(n: Value, m: Value) -> Vec<Difference> {
    if n != m {
        vec![Difference::change(n, m)]
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
            (None, Some(w)) => differences.push(Difference::Added(object_with(k, w))),
            (None, None) => unreachable!(
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

pub fn array_difference(left_vals: Vec<Value>, right_vals: Vec<Value>) -> Vec<Difference> {
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
    fn null_is_not_different_from_itself() {
        let left_value: Value = json!(null);
        let right_value: Value = json!(null);

        let difference = calculate(left_value, right_value);
        let empty: Vec<Difference> = Vec::new();

        assert_eq!(empty, difference)
    }

    #[test]
    fn difference_between_numbers() {
        let left_value: Value = json!(1);
        let right_value: Value = json!(2);

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::change(json!(1), json!(2))),
            difference
        )
    }

    #[test]
    fn difference_between_booleans() {
        let left_value: Value = json!(true);
        let right_value: Value = json!(false);

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::change(json!(true), json!(false))),
            difference
        )
    }

    #[test]
    fn same_boolean_value() {
        let left_value: Value = json!(true);
        let right_value: Value = json!(true);

        let difference = calculate(left_value, right_value);
        let empty: Vec<Difference> = Vec::new();

        assert_eq!(empty, difference)
    }

    #[test]
    fn array_with_missing_value() {
        let left_value: Value = json!([1, 2]);
        let right_value: Value = json!([1]);

        let difference = calculate(left_value, right_value);

        assert_eq!(vec!(Difference::Removed(json!(2))), difference)
    }

    #[test]
    fn array_with_surplus_value() {
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
            vec!(Difference::change(json!("foo"), json!("bar"))),
            difference
        )
    }

    #[test]
    fn second_object_has_surplus_elements() {
        let left_value: Value = json!({"a": "foo"});
        let right_value: Value = json!({"a": "foo", "b": "bar"});

        let difference = calculate(left_value, right_value);

        assert_eq!(vec!(Difference::Added(json!({"b": "bar"}))), difference)
    }

    #[test]
    fn second_object_has_missing_elements() {
        let left_value: Value = json!({"a": "foo", "b": "bar"});
        let right_value: Value = json!({"a": "foo"});

        let difference = calculate(left_value, right_value);

        assert_eq!(vec!(Difference::Removed(json!({"b": "bar"}))), difference)
    }

    #[test]
    fn object_with_value_compared_with_null() {
        let left_value: Value = json!({"a": "foo"});
        let right_value: Value = json!({ "a": null });

        let difference = calculate(left_value, right_value);

        assert_eq!(
            vec!(Difference::change(json!("foo"), json!(null))),
            difference
        )
    }
}
