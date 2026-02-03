import rustdash


# ==================== chunk Tests ====================

def test_chunk_even_split():
    assert rustdash.chunk([1, 2, 3, 4], 2) == [[1, 2], [3, 4]]

def test_chunk_uneven_split():
    assert rustdash.chunk([1, 2, 3, 4, 5], 2) == [[1, 2], [3, 4], [5]]

def test_chunk_size_one():
    assert rustdash.chunk([1, 2, 3], 1) == [[1], [2], [3]]

def test_chunk_size_larger_than_array():
    assert rustdash.chunk([1, 2, 3], 5) == [[1, 2, 3]]

def test_chunk_empty():
    assert rustdash.chunk([], 2) == []

def test_chunk_strings():
    assert rustdash.chunk(["a", "b", "c", "d"], 2) == [["a", "b"], ["c", "d"]]


# ==================== compact Tests ====================

def test_compact_with_none():
    assert rustdash.compact([1, None, 2, None, 3]) == [1, 2, 3]

def test_compact_all_values():
    assert rustdash.compact([1, 2, 3]) == [1, 2, 3]

def test_compact_all_none():
    assert rustdash.compact([None, None, None]) == []

def test_compact_empty():
    assert rustdash.compact([]) == []

def test_compact_strings():
    assert rustdash.compact(["hello", None, "world"]) == ["hello", "world"]


# ==================== flatten_deep Tests ====================

def test_flatten_deep_nested():
    assert rustdash.flatten_deep([[1, 2], [3, 4]]) == [1, 2, 3, 4]

def test_flatten_deep_single_level():
    assert rustdash.flatten_deep([[1], [2], [3]]) == [1, 2, 3]

def test_flatten_deep_empty_inner():
    assert rustdash.flatten_deep([[], [1, 2], []]) == [1, 2]

def test_flatten_deep_empty():
    assert rustdash.flatten_deep([]) == []

def test_flatten_deep_strings():
    assert rustdash.flatten_deep([["a", "b"], ["c"]]) == ["a", "b", "c"]

def test_flatten_deep_deeply_nested():
    assert rustdash.flatten_deep([[[1, 2], [3]], [[4], [5, 6]]]) == [1, 2, 3, 4, 5, 6]


# ==================== unique Tests ====================

def test_unique_basic():
    assert rustdash.unique([1, 2, 1, 2]) == [1, 2]

def test_unique_no_duplicates():
    assert rustdash.unique([1, 2, 3]) == [1, 2, 3]

def test_unique_all_same():
    assert rustdash.unique([5, 5, 5, 5]) == [5]

def test_unique_empty():
    assert rustdash.unique([]) == []

def test_unique_preserves_order():
    assert rustdash.unique([3, 1, 2, 1, 3]) == [3, 1, 2]

def test_unique_strings():
    assert rustdash.unique(["a", "b", "a", "c", "b"]) == ["a", "b", "c"]


# ==================== group_by Tests ====================

def test_group_by_basic():
    result = rustdash.group_by([1, 2, 3, 4, 5, 6], lambda x: x % 2)
    assert sorted(result[0]) == [2, 4, 6]
    assert sorted(result[1]) == [1, 3, 5]

def test_group_by_string_length():
    result = rustdash.group_by(["hi", "hey", "hello", "ok", "yes"], lambda w: len(w))
    assert sorted(result[2]) == sorted(["hi", "ok"])
    assert sorted(result[3]) == sorted(["hey", "yes"])
    assert result[5] == ["hello"]

def test_group_by_empty():
    result = rustdash.group_by([], lambda x: x % 2)
    assert result == {}

def test_group_by_single_group():
    result = rustdash.group_by([2, 4, 6], lambda x: x % 2)
    assert len(result) == 1
    assert result[0] == [2, 4, 6]


# ==================== map Tests ====================

def test_map_double():
    assert rustdash.map([1, 2, 3], lambda x: x * 2) == [2, 4, 6]

def test_map_to_string():
    assert rustdash.map([1, 2, 3], lambda x: str(x)) == ["1", "2", "3"]

def test_map_empty():
    assert rustdash.map([], lambda x: x * 2) == []

def test_map_square():
    assert rustdash.map([1, 2, 3, 4], lambda x: x * x) == [1, 4, 9, 16]

def test_map_negate():
    assert rustdash.map([1, -2, 3], lambda x: -x) == [-1, 2, -3]


# ==================== filter Tests ====================

def test_filter_even():
    assert rustdash.filter([1, 2, 3, 4, 5, 6], lambda x: x % 2 == 0) == [2, 4, 6]

def test_filter_positive():
    assert rustdash.filter([-1, 2, -3, 4], lambda x: x > 0) == [2, 4]

def test_filter_none_match():
    assert rustdash.filter([1, 3, 5], lambda x: x % 2 == 0) == []

def test_filter_all_match():
    assert rustdash.filter([2, 4, 6], lambda x: x % 2 == 0) == [2, 4, 6]

def test_filter_empty():
    assert rustdash.filter([], lambda x: x % 2 == 0) == []

def test_filter_strings():
    assert rustdash.filter(
        ["hello", "hi", "hey", "world"], lambda s: s.startswith("h")
    ) == ["hello", "hi", "hey"]


# ==================== find Tests ====================

def test_find_first_even():
    assert rustdash.find([1, 2, 3, 4], lambda x: x % 2 == 0) == 2

def test_find_no_match():
    assert rustdash.find([1, 3, 5], lambda x: x % 2 == 0) is None

def test_find_first_match_only():
    assert rustdash.find([1, 2, 4, 6], lambda x: x % 2 == 0) == 2

def test_find_empty():
    assert rustdash.find([], lambda x: x % 2 == 0) is None

def test_find_string():
    assert rustdash.find(["hello", "world", "hey"], lambda s: s.startswith("w")) == "world"


# ==================== sort_by Tests ====================

def test_sort_by_ascending():
    assert rustdash.sort_by([3, 1, 2], lambda x: x) == [1, 2, 3]

def test_sort_by_string_len():
    result = rustdash.sort_by(["hi", "hello", "hey"], lambda w: len(w))
    assert result == ["hi", "hey", "hello"]

def test_sort_by_already_sorted():
    assert rustdash.sort_by([1, 2, 3], lambda x: x) == [1, 2, 3]

def test_sort_by_empty():
    assert rustdash.sort_by([], lambda x: x) == []

def test_sort_by_single():
    assert rustdash.sort_by([42], lambda x: x) == [42]

def test_sort_by_dict_field():
    people = [
        {"name": "Charlie", "age": 30},
        {"name": "Alice", "age": 25},
        {"name": "Bob", "age": 28},
    ]
    result = rustdash.sort_by(people, lambda p: p["age"])
    assert result[0]["name"] == "Alice"
    assert result[1]["name"] == "Bob"
    assert result[2]["name"] == "Charlie"


# ==================== reduce Tests ====================

def test_reduce_sum():
    assert rustdash.reduce([1, 2, 3], lambda acc, x: acc + x, 0) == 6

def test_reduce_product():
    assert rustdash.reduce([1, 2, 3, 4], lambda acc, x: acc * x, 1) == 24

def test_reduce_string_concat():
    result = rustdash.reduce(
        ["hello", " ", "world"], lambda acc, s: acc + s, ""
    )
    assert result == "hello world"

def test_reduce_empty():
    assert rustdash.reduce([], lambda acc, x: acc + x, 0) == 0

def test_reduce_count():
    assert rustdash.reduce([1, 2, 3, 4, 5], lambda acc, _: acc + 1, 0) == 5

def test_reduce_max():
    result = rustdash.reduce(
        [3, 1, 4, 1, 5], lambda acc, x: x if x > acc else acc, float("-inf")
    )
    assert result == 5


# ==================== zip Tests ====================

def test_zip_basic():
    assert rustdash.zip([1, 2], [3, 4]) == [[1, 3], [2, 4]]

def test_zip_different_lengths():
    assert rustdash.zip([1, 2, 3], [4, 5]) == [[1, 4], [2, 5]]

def test_zip_empty():
    assert rustdash.zip([], [1, 2]) == []

def test_zip_single_elements():
    assert rustdash.zip([1], [2]) == [[1, 2]]

def test_zip_strings():
    assert rustdash.zip(["a", "b", "c"], ["x", "y", "z"]) == [
        ["a", "x"], ["b", "y"], ["c", "z"]
    ]


# ==================== intersection Tests ====================

def test_intersection_basic():
    assert rustdash.intersection([1, 2, 3], [2, 3, 4]) == [2, 3]

def test_intersection_no_overlap():
    assert rustdash.intersection([1, 2, 3], [4, 5, 6]) == []

def test_intersection_full_overlap():
    assert rustdash.intersection([1, 2, 3], [1, 2, 3]) == [1, 2, 3]

def test_intersection_duplicates():
    assert rustdash.intersection([1, 1, 2, 2], [2, 2, 3, 3]) == [2]

def test_intersection_empty():
    assert rustdash.intersection([], [1, 2, 3]) == []

def test_intersection_strings():
    assert rustdash.intersection(["a", "b", "c"], ["b", "c", "d"]) == ["b", "c"]
