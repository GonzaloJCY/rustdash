import math
import rustdash


# ==================== sum Tests ====================

def test_sum_basic():
    assert rustdash.sum([1, 2, 3]) == 6
    assert rustdash.sum([10, 20, 30]) == 60
    assert rustdash.sum([1, 2, 3, 4, 5]) == 15

def test_sum_single_element():
    assert rustdash.sum([5]) == 5
    assert rustdash.sum([100]) == 100

def test_sum_empty():
    assert rustdash.sum([]) == 0

def test_sum_negative_numbers():
    assert rustdash.sum([-1, -2, -3]) == -6
    assert rustdash.sum([-1, 2, -3, 4]) == 2

def test_sum_floats():
    assert abs(rustdash.sum([1.0, 2.0, 3.0]) - 6.0) < 1e-10
    assert abs(rustdash.sum([0.1, 0.2, 0.3]) - 0.6) < 1e-10


# ==================== sum_by Tests ====================

def test_sum_by_basic():
    items = [{"name": "a", "val": 1}, {"name": "b", "val": 2}, {"name": "c", "val": 3}]
    assert rustdash.sum_by(items, lambda x: x["val"]) == 6.0

def test_sum_by_empty():
    assert rustdash.sum_by([], lambda x: x) == 0.0

def test_sum_by_string_lengths():
    assert rustdash.sum_by(["hello", "world", "rust"], lambda w: len(w)) == 14.0


# ==================== max Tests ====================

def test_max_basic():
    assert rustdash.max([1, 5, 3]) == 5
    assert rustdash.max([10, 2, 8, 4]) == 10
    assert rustdash.max([1, 2, 3, 4, 5]) == 5

def test_max_single_element():
    assert rustdash.max([42]) == 42

def test_max_empty():
    assert rustdash.max([]) is None

def test_max_negative_numbers():
    assert rustdash.max([-5, -1, -10]) == -1
    assert rustdash.max([-1, 0, 1]) == 1

def test_max_duplicates():
    assert rustdash.max([5, 5, 5]) == 5
    assert rustdash.max([1, 5, 5, 3]) == 5

def test_max_floats():
    assert rustdash.max([1.5, 2.5, 0.5]) == 2.5
    assert rustdash.max([-1.0, -0.5, -2.0]) == -0.5


# ==================== max_by Tests ====================

def test_max_by_basic():
    items = [{"name": "a", "val": 1}, {"name": "b", "val": 5}, {"name": "c", "val": 3}]
    result = rustdash.max_by(items, lambda x: x["val"])
    assert result == {"name": "b", "val": 5}

def test_max_by_empty():
    assert rustdash.max_by([], lambda x: x) is None

def test_max_by_string_length():
    result = rustdash.max_by(["hi", "hello", "hey"], lambda w: len(w))
    assert result == "hello"


# ==================== min Tests ====================

def test_min_basic():
    assert rustdash.min([1, 5, 3]) == 1
    assert rustdash.min([10, 2, 8, 4]) == 2
    assert rustdash.min([5, 4, 3, 2, 1]) == 1

def test_min_single_element():
    assert rustdash.min([42]) == 42

def test_min_empty():
    assert rustdash.min([]) is None

def test_min_negative_numbers():
    assert rustdash.min([-5, -1, -10]) == -10
    assert rustdash.min([-1, 0, 1]) == -1

def test_min_duplicates():
    assert rustdash.min([5, 5, 5]) == 5
    assert rustdash.min([1, 5, 1, 3]) == 1

def test_min_floats():
    assert rustdash.min([1.5, 2.5, 0.5]) == 0.5
    assert rustdash.min([-1.0, -0.5, -2.0]) == -2.0


# ==================== min_by Tests ====================

def test_min_by_basic():
    items = [{"name": "a", "val": 3}, {"name": "b", "val": 1}, {"name": "c", "val": 5}]
    result = rustdash.min_by(items, lambda x: x["val"])
    assert result == {"name": "b", "val": 1}

def test_min_by_empty():
    assert rustdash.min_by([], lambda x: x) is None

def test_min_by_string_length():
    result = rustdash.min_by(["hello", "hi", "hey"], lambda w: len(w))
    assert result == "hi"


# ==================== mean Tests ====================

def test_mean_basic():
    assert abs(rustdash.mean([1.0, 2.0, 3.0]) - 2.0) < 1e-10
    assert abs(rustdash.mean([10.0, 20.0, 30.0]) - 20.0) < 1e-10

def test_mean_single_element():
    assert abs(rustdash.mean([5.0]) - 5.0) < 1e-10

def test_mean_empty():
    assert math.isnan(rustdash.mean([]))

def test_mean_negative_numbers():
    assert abs(rustdash.mean([-1.0, -2.0, -3.0]) - (-2.0)) < 1e-10
    assert abs(rustdash.mean([-2.0, 0.0, 2.0]) - 0.0) < 1e-10

def test_mean_floats():
    assert abs(rustdash.mean([1.0, 2.0, 3.0]) - 2.0) < 1e-10
    assert abs(rustdash.mean([2.5, 3.5, 4.0]) - (10.0 / 3.0)) < 1e-10

def test_mean_large_numbers():
    assert abs(rustdash.mean([1000000.0, 2000000.0, 3000000.0]) - 2000000.0) < 1e-10


# ==================== round Tests (single value) ====================

def test_round_basic():
    assert abs(rustdash.round(3.14159, 2) - 3.14) < 1e-10
    assert abs(rustdash.round(3.14159, 3) - 3.142) < 1e-10
    assert abs(rustdash.round(3.14159, 4) - 3.1416) < 1e-10

def test_round_zero_decimals():
    assert abs(rustdash.round(3.7, 0) - 4.0) < 1e-10
    assert abs(rustdash.round(3.4, 0) - 3.0) < 1e-10
    assert abs(rustdash.round(3.5, 0) - 4.0) < 1e-10

def test_round_negative_numbers():
    assert abs(rustdash.round(-3.14159, 2) - (-3.14)) < 1e-10
    assert abs(rustdash.round(-2.5, 0) - (-3.0)) < 1e-10

def test_round_already_rounded():
    assert abs(rustdash.round(3.14, 2) - 3.14) < 1e-10
    assert abs(rustdash.round(5.0, 3) - 5.0) < 1e-10

def test_round_small_decimals():
    assert abs(rustdash.round(0.123456789, 5) - 0.12346) < 1e-10
    assert abs(rustdash.round(0.999, 2) - 1.0) < 1e-10

def test_round_edge_cases():
    assert rustdash.round(0.0, 2) == 0.0
    assert rustdash.round(1.005, 2) == 1.00


# ==================== round Tests (array) ====================

def test_round_array_basic():
    result = rustdash.round([3.14159, 2.71828, 1.41421], 2)
    assert abs(result[0] - 3.14) < 1e-10
    assert abs(result[1] - 2.72) < 1e-10
    assert abs(result[2] - 1.41) < 1e-10

def test_round_array_zero_decimals():
    result = rustdash.round([3.7, 3.4, 3.5], 0)
    assert abs(result[0] - 4.0) < 1e-10
    assert abs(result[1] - 3.0) < 1e-10
    assert abs(result[2] - 4.0) < 1e-10

def test_round_array_negative_numbers():
    result = rustdash.round([-3.14159, -2.5], 2)
    assert abs(result[0] - (-3.14)) < 1e-10
    assert abs(result[1] - (-2.5)) < 1e-10

def test_round_array_empty():
    assert rustdash.round([], 2) == []

def test_round_array_single_element():
    result = rustdash.round([3.14159], 3)
    assert len(result) == 1
    assert abs(result[0] - 3.142) < 1e-10
