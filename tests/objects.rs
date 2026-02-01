use rustdash::core::objects::{get, get_all, has, has_all, keys, merge, omit, pick, values};
use serde_json::json;
use serde_json::Value;

// ==================== get Tests ====================

#[test]
fn test_get_top_level_key() {
    let obj = json!({"a": 1, "b": 2});
    assert_eq!(get(&obj, "a", Value::Null), json!(1));
}

#[test]
fn test_get_dotted_path() {
    let obj = json!({"user": {"name": "alice"}});
    assert_eq!(get(&obj, "user.name", Value::Null), json!("alice"));
}

#[test]
fn test_get_deeply_nested() {
    let obj = json!({"a": {"b": {"c": {"d": 42}}}});
    assert_eq!(get(&obj, "a.b.c.d", Value::Null), json!(42));
}

#[test]
fn test_get_array_index() {
    let obj = json!({"items": [10, 20, 30]});
    assert_eq!(get(&obj, "items[0]", Value::Null), json!(10));
    assert_eq!(get(&obj, "items[2]", Value::Null), json!(30));
}

#[test]
fn test_get_mixed_path_object_and_array() {
    let obj = json!({"users": [{"name": "alice"}, {"name": "bob"}]});
    assert_eq!(get(&obj, "users[0].name", Value::Null), json!("alice"));
    assert_eq!(get(&obj, "users[1].name", Value::Null), json!("bob"));
}

#[test]
fn test_get_nested_arrays() {
    let obj = json!({"matrix": [[1, 2], [3, 4]]});
    assert_eq!(get(&obj, "matrix[0][1]", Value::Null), json!(2));
    assert_eq!(get(&obj, "matrix[1][0]", Value::Null), json!(3));
}

#[test]
fn test_get_missing_key_returns_default() {
    let obj = json!({"a": 1});
    assert_eq!(get(&obj, "b", json!(42)), json!(42));
}

#[test]
fn test_get_missing_nested_returns_default() {
    let obj = json!({"a": {"b": 1}});
    assert_eq!(get(&obj, "a.x.y", json!("fallback")), json!("fallback"));
}

#[test]
fn test_get_index_out_of_bounds_returns_default() {
    let obj = json!({"items": [1, 2]});
    assert_eq!(get(&obj, "items[5]", json!(-1)), json!(-1));
}

#[test]
fn test_get_on_non_object_returns_default() {
    let obj = json!(42);
    assert_eq!(get(&obj, "a", json!("nope")), json!("nope"));
}

#[test]
fn test_get_returns_object() {
    let obj = json!({"a": {"b": {"c": 1}}});
    assert_eq!(get(&obj, "a.b", Value::Null), json!({"c": 1}));
}

#[test]
fn test_get_returns_array() {
    let obj = json!({"a": [1, 2, 3]});
    assert_eq!(get(&obj, "a", Value::Null), json!([1, 2, 3]));
}

// ==================== has Tests ====================

#[test]
fn test_has_existing_key() {
    let obj = json!({"a": 1, "b": 2});
    assert!(has(&obj, "a"));
}

#[test]
fn test_has_missing_key() {
    let obj = json!({"a": 1});
    assert!(!has(&obj, "z"));
}

#[test]
fn test_has_dotted_path_existing() {
    let obj = json!({"user": {"profile": {"name": "alice"}}});
    assert!(has(&obj, "user.profile.name"));
}

#[test]
fn test_has_dotted_path_missing() {
    let obj = json!({"user": {"profile": {"name": "alice"}}});
    assert!(!has(&obj, "user.profile.email"));
}

#[test]
fn test_has_array_index_existing() {
    let obj = json!({"items": [10, 20, 30]});
    assert!(has(&obj, "items[1]"));
}

#[test]
fn test_has_array_index_out_of_bounds() {
    let obj = json!({"items": [10, 20]});
    assert!(!has(&obj, "items[5]"));
}

#[test]
fn test_has_null_value_still_exists() {
    let obj = json!({"a": null});
    assert!(has(&obj, "a"));
}

#[test]
fn test_has_on_non_object() {
    let obj = json!("hello");
    assert!(!has(&obj, "a"));
}

#[test]
fn test_has_mixed_path() {
    let obj = json!({"users": [{"name": "alice"}]});
    assert!(has(&obj, "users[0].name"));
    assert!(!has(&obj, "users[0].email"));
}

// ==================== pick Tests ====================

#[test]
fn test_pick_subset() {
    let obj = json!({"a": 1, "b": 2, "c": 3});
    let result = pick(&obj, &["a", "c"]);
    assert_eq!(result, json!({"a": 1, "c": 3}));
}

#[test]
fn test_pick_with_missing_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = pick(&obj, &["a", "z"]);
    assert_eq!(result, json!({"a": 1}));
}

#[test]
fn test_pick_empty_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = pick(&obj, &[]);
    assert_eq!(result, json!({}));
}

#[test]
fn test_pick_all_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = pick(&obj, &["a", "b"]);
    assert_eq!(result, json!({"a": 1, "b": 2}));
}

#[test]
fn test_pick_preserves_nested_values() {
    let obj = json!({"name": "alice", "address": {"city": "NYC"}, "age": 30});
    let result = pick(&obj, &["name", "address"]);
    assert_eq!(result, json!({"name": "alice", "address": {"city": "NYC"}}));
}

#[test]
fn test_pick_on_non_object() {
    let obj = json!([1, 2, 3]);
    let result = pick(&obj, &["a"]);
    assert_eq!(result, json!({}));
}

// ==================== omit Tests ====================

#[test]
fn test_omit_single_key() {
    let obj = json!({"a": 1, "b": 2, "c": 3});
    let result = omit(&obj, &["b"]);
    assert_eq!(result, json!({"a": 1, "c": 3}));
}

#[test]
fn test_omit_multiple_keys() {
    let obj = json!({"a": 1, "b": 2, "c": 3, "d": 4});
    let result = omit(&obj, &["b", "d"]);
    assert_eq!(result, json!({"a": 1, "c": 3}));
}

#[test]
fn test_omit_missing_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = omit(&obj, &["z"]);
    assert_eq!(result, json!({"a": 1, "b": 2}));
}

#[test]
fn test_omit_all_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = omit(&obj, &["a", "b"]);
    assert_eq!(result, json!({}));
}

#[test]
fn test_omit_empty_keys() {
    let obj = json!({"a": 1, "b": 2});
    let result = omit(&obj, &[]);
    assert_eq!(result, json!({"a": 1, "b": 2}));
}

#[test]
fn test_omit_on_non_object() {
    let obj = json!("hello");
    let result = omit(&obj, &["a"]);
    assert_eq!(result, json!({}));
}

// ==================== merge Tests ====================

#[test]
fn test_merge_two_objects() {
    let result = merge(&[json!({"a": 1}), json!({"b": 2})]);
    assert_eq!(result, json!({"a": 1, "b": 2}));
}

#[test]
fn test_merge_overlapping_keys_last_wins() {
    let result = merge(&[json!({"a": 1}), json!({"a": 2})]);
    assert_eq!(result, json!({"a": 2}));
}

#[test]
fn test_merge_three_objects() {
    let result = merge(&[json!({"a": 1}), json!({"b": 2}), json!({"c": 3})]);
    assert_eq!(result, json!({"a": 1, "b": 2, "c": 3}));
}

#[test]
fn test_merge_empty_slice() {
    let result = merge(&[]);
    assert_eq!(result, json!({}));
}

#[test]
fn test_merge_single_object() {
    let result = merge(&[json!({"a": 1})]);
    assert_eq!(result, json!({"a": 1}));
}

#[test]
fn test_merge_shallow_no_deep() {
    let result = merge(&[
        json!({"config": {"debug": true, "level": 1}}),
        json!({"config": {"debug": false}}),
    ]);
    // Shallow merge: the entire "config" key is replaced, not deep-merged
    assert_eq!(result, json!({"config": {"debug": false}}));
}

#[test]
fn test_merge_mixed_value_types() {
    let result = merge(&[json!({"a": 1}), json!({"a": "hello"})]);
    assert_eq!(result, json!({"a": "hello"}));
}

// ==================== keys Tests ====================

#[test]
fn test_keys_basic() {
    let obj = json!({"a": 1, "b": 2, "c": 3});
    let mut result = keys(&obj);
    result.sort();
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_keys_empty_object() {
    let obj = json!({});
    assert!(keys(&obj).is_empty());
}

#[test]
fn test_keys_non_object() {
    let obj = json!([1, 2, 3]);
    assert!(keys(&obj).is_empty());
}

#[test]
fn test_keys_single_key() {
    let obj = json!({"only": true});
    assert_eq!(keys(&obj), vec!["only"]);
}

// ==================== values Tests ====================

#[test]
fn test_values_basic() {
    let obj = json!({"a": 1, "b": 2});
    let result = values(&obj);
    // Values may come in any order, so check length and membership
    assert_eq!(result.len(), 2);
    assert!(result.contains(&json!(1)));
    assert!(result.contains(&json!(2)));
}

#[test]
fn test_values_empty_object() {
    let obj = json!({});
    assert!(values(&obj).is_empty());
}

#[test]
fn test_values_non_object() {
    let obj = json!(42);
    assert!(values(&obj).is_empty());
}

#[test]
fn test_values_mixed_types() {
    let obj = json!({"name": "alice", "age": 30, "active": true});
    let result = values(&obj);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&json!("alice")));
    assert!(result.contains(&json!(30)));
    assert!(result.contains(&json!(true)));
}

#[test]
fn test_values_preserves_nested() {
    let obj = json!({"data": {"nested": true}});
    let result = values(&obj);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], json!({"nested": true}));
}

// ==================== get_all Tests ====================

#[test]
fn test_get_all_wildcard_basic() {
    let obj = json!({"users": [{"name": "alice"}, {"name": "bob"}]});
    assert_eq!(
        get_all(&obj, "users[*].name"),
        vec![json!("alice"), json!("bob")]
    );
}

#[test]
fn test_get_all_wildcard_numbers() {
    let obj = json!({"scores": [10, 20, 30]});
    assert_eq!(
        get_all(&obj, "scores[*]"),
        vec![json!(10), json!(20), json!(30)]
    );
}

#[test]
fn test_get_all_nested_wildcards() {
    let obj = json!({"matrix": [[1, 2], [3, 4]]});
    assert_eq!(
        get_all(&obj, "matrix[*][*]"),
        vec![json!(1), json!(2), json!(3), json!(4)]
    );
}

#[test]
fn test_get_all_wildcard_deep_path() {
    let obj = json!({
        "departments": [
            {"employees": [{"name": "alice"}, {"name": "bob"}]},
            {"employees": [{"name": "charlie"}]}
        ]
    });
    assert_eq!(
        get_all(&obj, "departments[*].employees[*].name"),
        vec![json!("alice"), json!("bob"), json!("charlie")]
    );
}

#[test]
fn test_get_all_missing_field_skipped() {
    let obj = json!({
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob"}
        ]
    });
    // bob has no email → only alice's email is collected
    assert_eq!(get_all(&obj, "users[*].email"), vec![json!("a@b.com")]);
}

#[test]
fn test_get_all_empty_array() {
    let obj = json!({"users": []});
    assert!(get_all(&obj, "users[*].name").is_empty());
}

#[test]
fn test_get_all_no_wildcard_acts_like_get() {
    let obj = json!({"a": {"b": 1}});
    assert_eq!(get_all(&obj, "a.b"), vec![json!(1)]);
}

#[test]
fn test_get_all_path_not_found() {
    let obj = json!({"a": 1});
    assert!(get_all(&obj, "z[*].x").is_empty());
}

#[test]
fn test_get_all_returns_objects() {
    let obj = json!({
        "items": [
            {"meta": {"id": 1}},
            {"meta": {"id": 2}}
        ]
    });
    assert_eq!(
        get_all(&obj, "items[*].meta"),
        vec![json!({"id": 1}), json!({"id": 2})]
    );
}

// ==================== has_all Tests ====================

#[test]
fn test_has_all_all_present() {
    let obj = json!({
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob", "email": "b@c.com"}
        ]
    });
    assert!(has_all(&obj, "users[*].email"));
}

#[test]
fn test_has_all_some_missing() {
    let obj = json!({
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob"}
        ]
    });
    assert!(!has_all(&obj, "users[*].email"));
}

#[test]
fn test_has_all_empty_array_returns_false() {
    let obj = json!({"users": []});
    assert!(!has_all(&obj, "users[*].name"));
}

#[test]
fn test_has_all_nested_wildcards() {
    let obj = json!({
        "matrix": [[1, 2], [3, 4]]
    });
    assert!(has_all(&obj, "matrix[*][*]"));
}

#[test]
fn test_has_all_nested_wildcards_uneven() {
    let obj = json!({
        "matrix": [[1, 2], []]
    });
    // Second inner array is empty → no elements to match → false
    assert!(!has_all(&obj, "matrix[*][*]"));
}

#[test]
fn test_has_all_no_wildcard_acts_like_has() {
    let obj = json!({"a": {"b": 1}});
    assert!(has_all(&obj, "a.b"));
    assert!(!has_all(&obj, "a.z"));
}

#[test]
fn test_has_all_deep_path() {
    let obj = json!({
        "departments": [
            {"employees": [{"name": "alice"}, {"name": "bob"}]},
            {"employees": [{"name": "charlie"}]}
        ]
    });
    assert!(has_all(&obj, "departments[*].employees[*].name"));
}

#[test]
fn test_has_all_null_values_still_exist() {
    let obj = json!({"items": [{"val": null}, {"val": null}]});
    assert!(has_all(&obj, "items[*].val"));
}
