// // use rustdash::core::numbers::{max_, max_by, mean, min_, min_by, round, sum_, sum_by};
use rustdash::core::numbers::{max_, max_by_, mean_, min_, min_by_, round_, sum_, sum_by_};

// ==================== sum_ Tests ====================
#[test]
fn test_sum_basic() {
    assert_eq!(sum_(&[1, 2, 3]), 6);
    assert_eq!(sum_(&[10, 20, 30]), 60);
    assert_eq!(sum_(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_sum_single_element() {
    assert_eq!(sum_(&[5]), 5);
    assert_eq!(sum_(&[100]), 100);
}

#[test]
fn test_sum_empty() {
    let empty: &[i32] = &[];
    assert_eq!(sum_(empty), 0);
}

#[test]
fn test_sum_negative_numbers() {
    assert_eq!(sum_(&[-1, -2, -3]), -6);
    assert_eq!(sum_(&[-1, 2, -3, 4]), 2);
}

#[test]
fn test_sum_floats() {
    assert!((sum_(&[1.0, 2.0, 3.0]) as f64 - 6.0).abs() < f64::EPSILON);
    assert!((sum_(&[0.1, 0.2, 0.3]) as f64 - 0.6).abs() < 1e-10);
}

// ==================== sum_by Tests ====================

#[test]
fn test_sum_by_basic() {
    let items = vec![("a", 1), ("b", 2), ("c", 3)];
    assert_eq!(sum_by_(&items, |item| item.1), 6);
}

#[test]
fn test_sum_by_struct() {
    #[derive(Debug)]
    struct Item {
        n: i32,
    }
    let items = vec![Item { n: 1 }, Item { n: 2 }, Item { n: 3 }];
    assert_eq!(sum_by_(&items, |item| item.n), 6);
}

#[test]
fn test_sum_by_empty() {
    let items: Vec<(String, i32)> = vec![];
    assert_eq!(sum_by_(&items, |item| item.1), 0);
}

#[test]
fn test_sum_by_string_lengths() {
    let words = vec!["hello", "world", "rust"];
    assert_eq!(sum_by_(&words, |w| w.len() as i32), 14);
}

// ==================== max_ Tests ====================

#[test]
fn test_max_basic() {
    assert_eq!(max_(&[1, 5, 3]), Some(5));
    assert_eq!(max_(&[10, 2, 8, 4]), Some(10));
    assert_eq!(max_(&[1, 2, 3, 4, 5]), Some(5));
}

#[test]
fn test_max_single_element() {
    assert_eq!(max_(&[42]), Some(42));
}

#[test]
fn test_max_empty() {
    let empty: &[i32] = &[];
    assert_eq!(max_(empty), None);
}

#[test]
fn test_max_negative_numbers() {
    assert_eq!(max_(&[-5, -1, -10]), Some(-1));
    assert_eq!(max_(&[-1, 0, 1]), Some(1));
}

#[test]
fn test_max_duplicates() {
    assert_eq!(max_(&[5, 5, 5]), Some(5));
    assert_eq!(max_(&[1, 5, 5, 3]), Some(5));
}

#[test]
fn test_max_floats() {
    assert_eq!(max_(&[1.5, 2.5, 0.5]), Some(2.5));
    assert_eq!(max_(&[-1.0, -0.5, -2.0]), Some(-0.5));
}

// ==================== max_by Tests ====================

#[test]
fn test_max_by_basic() {
    let items = vec![("a", 1), ("b", 5), ("c", 3)];
    assert_eq!(max_by_(&items, |item| item.1), Some(&("b", 5)));
}

#[test]
fn test_max_by_struct() {
    #[derive(Debug, PartialEq)]
    struct Item {
        v: i32,
    }
    let items = vec![Item { v: 1 }, Item { v: 5 }, Item { v: 3 }];
    assert_eq!(max_by_(&items, |item| item.v), Some(&Item { v: 5 }));
}

#[test]
fn test_max_by_empty() {
    let items: Vec<(String, i32)> = vec![];
    assert_eq!(max_by_(&items, |item| item.1), None);
}

#[test]
fn test_max_by_string_length() {
    let words = vec!["hi", "hello", "hey"];
    assert_eq!(max_by_(&words, |w| w.len()), Some(&"hello"));
}

// ==================== min_ Tests ====================

#[test]
fn test_min_basic() {
    assert_eq!(min_(&[1, 5, 3]), Some(1));
    assert_eq!(min_(&[10, 2, 8, 4]), Some(2));
    assert_eq!(min_(&[5, 4, 3, 2, 1]), Some(1));
}

#[test]
fn test_min_single_element() {
    assert_eq!(min_(&[42]), Some(42));
}

#[test]
fn test_min_empty() {
    let empty: &[i32] = &[];
    assert_eq!(min_(empty), None);
}

#[test]
fn test_min_negative_numbers() {
    assert_eq!(min_(&[-5, -1, -10]), Some(-10));
    assert_eq!(min_(&[-1, 0, 1]), Some(-1));
}

#[test]
fn test_min_duplicates() {
    assert_eq!(min_(&[5, 5, 5]), Some(5));
    assert_eq!(min_(&[1, 5, 1, 3]), Some(1));
}

#[test]
fn test_min_floats() {
    assert_eq!(min_(&[1.5, 2.5, 0.5]), Some(0.5));
    assert_eq!(min_(&[-1.0, -0.5, -2.0]), Some(-2.0));
}

// ==================== min_by Tests ====================

#[test]
fn test_min_by_basic() {
    let items = vec![("a", 3), ("b", 1), ("c", 5)];
    assert_eq!(min_by_(&items, |item| item.1), Some(&("b", 1)));
}

#[test]
fn test_min_by_struct() {
    #[derive(Debug, PartialEq)]
    struct Item {
        v: i32,
    }
    let items = vec![Item { v: 3 }, Item { v: 1 }, Item { v: 5 }];
    assert_eq!(min_by_(&items, |item| item.v), Some(&Item { v: 1 }));
}

#[test]
fn test_min_by_empty() {
    let items: Vec<(String, i32)> = vec![];
    assert_eq!(min_by_(&items, |item| item.1), None);
}

#[test]
fn test_min_by_string_length() {
    let words = vec!["hello", "hi", "hey"];
    assert_eq!(min_by_(&words, |w| w.len()), Some(&"hi"));
}

// ==================== mean Tests ====================

#[test]
fn test_mean_basic() {
    assert!((mean_(&[1, 2, 3]) - 2.0).abs() < f64::EPSILON);
    assert!((mean_(&[10, 20, 30]) - 20.0).abs() < f64::EPSILON);
}

#[test]
fn test_mean_single_element() {
    assert!((mean_(&[5]) - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_mean_empty() {
    let empty: &[i32] = &[];
    assert!(mean_(empty).is_nan());
}

#[test]
fn test_mean_negative_numbers() {
    assert!((mean_(&[-1, -2, -3]) - (-2.0)).abs() < f64::EPSILON);
    assert!((mean_(&[-2, 0, 2]) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_mean_floats() {
    assert!((mean_(&[1.0, 2.0, 3.0]) - 2.0).abs() < f64::EPSILON);
    assert!((mean_(&[2.5, 3.5, 4.0]) - (10.0 / 3.0)).abs() < 1e-10);
}

#[test]
fn test_mean_large_numbers() {
    assert!((mean_(&[1000000, 2000000, 3000000]) - 2000000.0).abs() < f64::EPSILON);
}

// ==================== round_ Tests (single value) ====================

#[test]
fn test_round_basic() {
    assert!((round_(3.14159, 2) - 3.14).abs() < f64::EPSILON);
    assert!((round_(3.14159, 3) - 3.142).abs() < f64::EPSILON);
    assert!((round_(3.14159, 4) - 3.1416).abs() < f64::EPSILON);
}

#[test]
fn test_round_zero_decimals() {
    assert!((round_(3.7, 0) - 4.0).abs() < f64::EPSILON);
    assert!((round_(3.4, 0) - 3.0).abs() < f64::EPSILON);
    assert!((round_(3.5, 0) - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_round_negative_numbers() {
    assert!((round_(-3.14159, 2) - (-3.14)).abs() < f64::EPSILON);
    assert!((round_(-2.5, 0) - (-3.0)).abs() < f64::EPSILON);
}

#[test]
fn test_round_already_rounded() {
    assert!((round_(3.14, 2) - 3.14).abs() < f64::EPSILON);
    assert!((round_(5.0, 3) - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_round_small_decimals() {
    assert!((round_(0.123456789, 5) - 0.12346).abs() < f64::EPSILON);
    assert!((round_(0.999, 2) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_round_edge_cases() {
    assert!(round_(0.0, 2) == 0.0);
    assert!(round_(1.005, 2) == 1.00);
}

// ==================== round_ Tests (arrays) ====================

#[test]
fn test_round_array_basic() {
    let result = round_(&[3.14159, 2.71828, 1.41421][..], 2);
    assert!((result[0] - 3.14).abs() < f64::EPSILON);
    assert!((result[1] - 2.72).abs() < f64::EPSILON);
    assert!((result[2] - 1.41).abs() < f64::EPSILON);
}

#[test]
fn test_round_array_zero_decimals() {
    let result = round_(&[3.7, 3.4, 3.5][..], 0);
    assert!((result[0] - 4.0).abs() < f64::EPSILON);
    assert!((result[1] - 3.0).abs() < f64::EPSILON);
    assert!((result[2] - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_round_array_negative_numbers() {
    let result = round_(&[-3.14159, -2.5][..], 2);
    assert!((result[0] - (-3.14)).abs() < f64::EPSILON);
    assert!((result[1] - (-2.5)).abs() < f64::EPSILON);
}

#[test]
fn test_round_array_empty() {
    let empty: &[f64] = &[];
    let result = round_(empty, 2);
    assert!(result.is_empty());
}

#[test]
fn test_round_array_single_element() {
    let result = round_(&[3.14159][..], 3);
    assert_eq!(result.len(), 1);
    assert!((result[0] - 3.142).abs() < f64::EPSILON);
}
