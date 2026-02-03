use rustdash::core::numbers::{max_, mean_, min_, round_, sum_};

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
