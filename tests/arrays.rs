use rustdash::core::arrays::{
    chunk, compact, filter, find, flatten_deep, group_by, intersection, map, reduce, sort_by,
    unique, zip,
};

// ==================== chunk Tests ====================

#[test]
fn test_chunk_even_split() {
    assert_eq!(chunk(&[1, 2, 3, 4], 2), vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn test_chunk_uneven_split() {
    assert_eq!(
        chunk(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

#[test]
fn test_chunk_size_one() {
    assert_eq!(chunk(&[1, 2, 3], 1), vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn test_chunk_size_larger_than_array() {
    assert_eq!(chunk(&[1, 2, 3], 5), vec![vec![1, 2, 3]]);
}

#[test]
fn test_chunk_empty() {
    let empty: &[i32] = &[];
    let result: Vec<Vec<i32>> = chunk(empty, 2);
    assert!(result.is_empty());
}

#[test]
fn test_chunk_strings() {
    assert_eq!(
        chunk(&["a", "b", "c", "d"], 2),
        vec![vec!["a", "b"], vec!["c", "d"]]
    );
}

// ==================== compact Tests ====================

#[test]
fn test_compact_with_none() {
    assert_eq!(
        compact(&[Some(1), None, Some(2), None, Some(3)]),
        vec![1, 2, 3]
    );
}

#[test]
fn test_compact_all_some() {
    assert_eq!(compact(&[Some(1), Some(2), Some(3)]), vec![1, 2, 3]);
}

#[test]
fn test_compact_all_none() {
    let input: &[Option<i32>] = &[None, None, None];
    let result: Vec<i32> = compact(input);
    assert!(result.is_empty());
}

#[test]
fn test_compact_empty() {
    let empty: &[Option<i32>] = &[];
    let result: Vec<i32> = compact(empty);
    assert!(result.is_empty());
}

#[test]
fn test_compact_strings() {
    assert_eq!(
        compact(&[Some("hello"), None, Some("world")]),
        vec!["hello", "world"]
    );
}

// ==================== flatten_deep Tests ====================

#[test]
fn test_flatten_deep_nested() {
    assert_eq!(
        flatten_deep(&vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_flatten_deep_single_level() {
    assert_eq!(
        flatten_deep(&vec![vec![1], vec![2], vec![3]]),
        vec![1, 2, 3]
    );
}

#[test]
fn test_flatten_deep_empty_inner() {
    let input: Vec<Vec<i32>> = vec![vec![], vec![1, 2], vec![]];
    assert_eq!(flatten_deep(&input), vec![1, 2]);
}

#[test]
fn test_flatten_deep_empty() {
    let input: Vec<Vec<i32>> = vec![];
    let result: Vec<i32> = flatten_deep(&input);
    assert!(result.is_empty());
}

#[test]
fn test_flatten_deep_strings() {
    assert_eq!(
        flatten_deep(&vec![vec!["a", "b"], vec!["c"]]),
        vec!["a", "b", "c"]
    );
}

//==================== unique Tests ====================

#[test]
fn test_unique_basic() {
    assert_eq!(unique(&[1, 2, 1, 2]), vec![1, 2]);
}

#[test]
fn test_unique_no_duplicates() {
    assert_eq!(unique(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_unique_all_same() {
    assert_eq!(unique(&[5, 5, 5, 5]), vec![5]);
}

#[test]
fn test_unique_empty() {
    let empty: &[i32] = &[];
    let result: Vec<i32> = unique(empty);
    assert!(result.is_empty());
}

#[test]
fn test_unique_preserves_order() {
    assert_eq!(unique(&[3, 1, 2, 1, 3]), vec![3, 1, 2]);
}

#[test]
fn test_unique_strings() {
    assert_eq!(unique(&["a", "b", "a", "c", "b"]), vec!["a", "b", "c"]);
}

// ==================== group_by_ Tests ====================

#[test]
fn test_group_by_basic() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let result = group_by(&items, |x| x % 2);
    assert_eq!(result[&0], vec![&2, &4, &6]);
    assert_eq!(result[&1], vec![&1, &3, &5]);
}

#[test]
fn test_group_by_string_length() {
    let words = vec!["hi", "hey", "hello", "ok", "yes"];
    let result = group_by(&words, |w| w.len());
    assert_eq!(result[&2], vec![&"hi", &"ok"]);
    assert_eq!(result[&3], vec![&"hey", &"yes"]);
    assert_eq!(result[&5], vec![&"hello"]);
}

#[test]
fn test_group_by_empty() {
    let items: Vec<i32> = vec![];
    let result = group_by(&items, |x| x % 2);
    assert!(result.is_empty());
}

#[test]
fn test_group_by_single_group() {
    let items = vec![2, 4, 6];
    let result = group_by(&items, |x| x % 2);
    assert_eq!(result.len(), 1);
    assert_eq!(result[&0], vec![&2, &4, &6]);
}

// ==================== map_ Tests ====================

#[test]
fn test_map_double() {
    assert_eq!(map(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
}

#[test]
fn test_map_to_string() {
    assert_eq!(map(&[1, 2, 3], |x| x.to_string()), vec!["1", "2", "3"]);
}

#[test]
fn test_map_empty() {
    let empty: &[i32] = &[];
    let result: Vec<i32> = map(empty, |x| x * 2);
    assert!(result.is_empty());
}

#[test]
fn test_map_square() {
    assert_eq!(map(&[1, 2, 3, 4], |x| x * x), vec![1, 4, 9, 16]);
}

#[test]
fn test_map_negate() {
    assert_eq!(map(&[1, -2, 3], |x| -x), vec![-1, 2, -3]);
}

// ==================== filter_ Tests ====================

#[test]
fn test_filter_even() {
    assert_eq!(
        filter(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0),
        vec![&2, &4, &6]
    );
}

#[test]
fn test_filter_positive() {
    assert_eq!(filter(&[-1, 2, -3, 4], |x| *x > 0), vec![&2, &4]);
}

#[test]
fn test_filter_none_match() {
    let result = filter(&[1, 3, 5], |x| x % 2 == 0);
    assert!(result.is_empty());
}

#[test]
fn test_filter_all_match() {
    assert_eq!(filter(&[2, 4, 6], |x| x % 2 == 0), vec![&2, &4, &6]);
}

#[test]
fn test_filter_empty() {
    let empty: &[i32] = &[];
    let result = filter(empty, |x| x % 2 == 0);
    assert!(result.is_empty());
}

#[test]
fn test_filter_strings() {
    assert_eq!(
        filter(&["hello", "hi", "hey", "world"], |s| s.starts_with('h')),
        vec![&"hello", &"hi", &"hey"]
    );
}

// ==================== find_ Tests ====================

#[test]
fn test_find_first_even() {
    assert_eq!(find(&[1, 2, 3, 4], |x| x % 2 == 0), Some(&2));
}

#[test]
fn test_find_no_match() {
    assert_eq!(find(&[1, 3, 5], |x| x % 2 == 0), None);
}

#[test]
fn test_find_first_match_only() {
    assert_eq!(find(&[1, 2, 4, 6], |x| x % 2 == 0), Some(&2));
}

#[test]
fn test_find_empty() {
    let empty: &[i32] = &[];
    assert_eq!(find(empty, |x| x % 2 == 0), None);
}

#[test]
fn test_find_string() {
    let words = vec!["hello", "world", "hey"];
    assert_eq!(find(&words, |s| s.starts_with('w')), Some(&"world"));
}

// ==================== sort_by_ Tests ====================

#[test]
fn test_sort_by_ascending() {
    assert_eq!(sort_by(&[3, 1, 2], |x| *x), vec![&1, &2, &3]);
}

#[test]
fn test_sort_by_descending_string_len() {
    let words = vec!["hi", "hello", "hey"];
    let result = sort_by(&words, |w| w.len());
    assert_eq!(result, vec![&"hi", &"hey", &"hello"]);
}

#[test]
fn test_sort_by_already_sorted() {
    assert_eq!(sort_by(&[1, 2, 3], |x| *x), vec![&1, &2, &3]);
}

#[test]
fn test_sort_by_empty() {
    let empty: &[i32] = &[];
    let result: Vec<&i32> = sort_by(empty, |x| *x);
    assert!(result.is_empty());
}

#[test]
fn test_sort_by_single() {
    assert_eq!(sort_by(&[42], |x| *x), vec![&42]);
}

#[test]
fn test_sort_by_struct() {
    #[derive(Debug, PartialEq)]
    struct Person {
        name: &'static str,
        age: u32,
    }
    let people = vec![
        Person {
            name: "Charlie",
            age: 30,
        },
        Person {
            name: "Alice",
            age: 25,
        },
        Person {
            name: "Bob",
            age: 28,
        },
    ];
    let result = sort_by(&people, |p| p.age);
    assert_eq!(result[0].name, "Alice");
    assert_eq!(result[1].name, "Bob");
    assert_eq!(result[2].name, "Charlie");
}

// ==================== reduce_ Tests ====================

#[test]
fn test_reduce_sum() {
    assert_eq!(reduce(&[1, 2, 3], |acc, x| acc + x, 0), 6);
}

#[test]
fn test_reduce_product() {
    assert_eq!(reduce(&[1, 2, 3, 4], |acc, x| acc * x, 1), 24);
}

#[test]
fn test_reduce_string_concat() {
    let result = reduce(
        &["hello", " ", "world"],
        |acc, s| format!("{}{}", acc, s),
        String::new(),
    );
    assert_eq!(result, "hello world");
}

#[test]
fn test_reduce_empty() {
    assert_eq!(reduce(&[] as &[i32], |acc, x| acc + x, 0), 0);
}

#[test]
fn test_reduce_count() {
    assert_eq!(reduce(&[1, 2, 3, 4, 5], |acc, _| acc + 1, 0), 5);
}

#[test]
fn test_reduce_max() {
    assert_eq!(
        reduce(
            &[3, 1, 4, 1, 5],
            |acc, x| if *x > acc { *x } else { acc },
            i32::MIN
        ),
        5
    );
}

// ==================== zip_ Tests ====================

#[test]
fn test_zip_basic() {
    assert_eq!(zip(&[1, 2], &[3, 4]), vec![(1, 3), (2, 4)]);
}

#[test]
fn test_zip_different_lengths() {
    assert_eq!(zip(&[1, 2, 3], &[4, 5]), vec![(1, 4), (2, 5)]);
}

#[test]
fn test_zip_empty() {
    let empty: &[i32] = &[];
    let result: Vec<(i32, i32)> = zip(empty, &[1, 2]);
    assert!(result.is_empty());
}

#[test]
fn test_zip_single_elements() {
    assert_eq!(zip(&[1], &[2]), vec![(1, 2)]);
}

#[test]
fn test_zip_strings() {
    assert_eq!(
        zip(&["a", "b", "c"], &["x", "y", "z"]),
        vec![("a", "x"), ("b", "y"), ("c", "z")]
    );
}

// ==================== intersection Tests ====================

#[test]
fn test_intersection_basic() {
    let result = intersection(&[1, 2, 3], &[2, 3, 4]);
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn test_intersection_no_overlap() {
    let result = intersection(&[1, 2, 3], &[4, 5, 6]);
    assert!(result.is_empty());
}

#[test]
fn test_intersection_full_overlap() {
    let result = intersection(&[1, 2, 3], &[1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_intersection_duplicates() {
    let result = intersection(&[1, 1, 2, 2], &[2, 2, 3, 3]);
    assert_eq!(result, vec![2]);
}

#[test]
fn test_intersection_empty() {
    let empty: &[i32] = &[];
    let result = intersection(empty, &[1, 2, 3]);
    assert!(result.is_empty());
}

#[test]
fn test_intersection_strings() {
    let result = intersection(&["a", "b", "c"], &["b", "c", "d"]);
    assert_eq!(result, vec!["b", "c"]);
}
