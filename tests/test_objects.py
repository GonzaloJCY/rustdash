import rustdash


# ==================== get Tests ====================

def test_get_top_level_key():
    assert rustdash.get({"a": 1, "b": 2}, "a", None) == 1

def test_get_dotted_path():
    assert rustdash.get({"user": {"name": "alice"}}, "user.name", None) == "alice"

def test_get_deeply_nested():
    assert rustdash.get({"a": {"b": {"c": {"d": 42}}}}, "a.b.c.d", None) == 42

def test_get_array_index():
    obj = {"items": [10, 20, 30]}
    assert rustdash.get(obj, "items[0]", None) == 10
    assert rustdash.get(obj, "items[2]", None) == 30

def test_get_mixed_path_object_and_array():
    obj = {"users": [{"name": "alice"}, {"name": "bob"}]}
    assert rustdash.get(obj, "users[0].name", None) == "alice"
    assert rustdash.get(obj, "users[1].name", None) == "bob"

def test_get_nested_arrays():
    obj = {"matrix": [[1, 2], [3, 4]]}
    assert rustdash.get(obj, "matrix[0][1]", None) == 2
    assert rustdash.get(obj, "matrix[1][0]", None) == 3

def test_get_missing_key_returns_default():
    assert rustdash.get({"a": 1}, "b", 42) == 42

def test_get_missing_nested_returns_default():
    assert rustdash.get({"a": {"b": 1}}, "a.x.y", "fallback") == "fallback"

def test_get_index_out_of_bounds_returns_default():
    assert rustdash.get({"items": [1, 2]}, "items[5]", -1) == -1

def test_get_returns_object():
    obj = {"a": {"b": {"c": 1}}}
    assert rustdash.get(obj, "a.b", None) == {"c": 1}

def test_get_returns_array():
    obj = {"a": [1, 2, 3]}
    assert rustdash.get(obj, "a", None) == [1, 2, 3]


# ==================== has Tests ====================

def test_has_existing_key():
    assert rustdash.has({"a": 1, "b": 2}, "a") is True

def test_has_missing_key():
    assert rustdash.has({"a": 1}, "z") is False

def test_has_dotted_path_existing():
    assert rustdash.has({"user": {"profile": {"name": "alice"}}}, "user.profile.name") is True

def test_has_dotted_path_missing():
    assert rustdash.has({"user": {"profile": {"name": "alice"}}}, "user.profile.email") is False

def test_has_array_index_existing():
    assert rustdash.has({"items": [10, 20, 30]}, "items[1]") is True

def test_has_array_index_out_of_bounds():
    assert rustdash.has({"items": [10, 20]}, "items[5]") is False

def test_has_null_value_still_exists():
    assert rustdash.has({"a": None}, "a") is True

def test_has_mixed_path():
    obj = {"users": [{"name": "alice"}]}
    assert rustdash.has(obj, "users[0].name") is True
    assert rustdash.has(obj, "users[0].email") is False


# ==================== pick Tests ====================

def test_pick_subset():
    assert rustdash.pick({"a": 1, "b": 2, "c": 3}, ["a", "c"]) == {"a": 1, "c": 3}

def test_pick_with_missing_keys():
    assert rustdash.pick({"a": 1, "b": 2}, ["a", "z"]) == {"a": 1}

def test_pick_empty_keys():
    assert rustdash.pick({"a": 1, "b": 2}, []) == {}

def test_pick_all_keys():
    assert rustdash.pick({"a": 1, "b": 2}, ["a", "b"]) == {"a": 1, "b": 2}

def test_pick_preserves_nested_values():
    obj = {"name": "alice", "address": {"city": "NYC"}, "age": 30}
    result = rustdash.pick(obj, ["name", "address"])
    assert result == {"name": "alice", "address": {"city": "NYC"}}

def test_pick_on_non_object():
    assert rustdash.pick([1, 2, 3], ["a"]) == {}


# ==================== omit Tests ====================

def test_omit_single_key():
    assert rustdash.omit({"a": 1, "b": 2, "c": 3}, ["b"]) == {"a": 1, "c": 3}

def test_omit_multiple_keys():
    assert rustdash.omit({"a": 1, "b": 2, "c": 3, "d": 4}, ["b", "d"]) == {"a": 1, "c": 3}

def test_omit_missing_keys():
    assert rustdash.omit({"a": 1, "b": 2}, ["z"]) == {"a": 1, "b": 2}

def test_omit_all_keys():
    assert rustdash.omit({"a": 1, "b": 2}, ["a", "b"]) == {}

def test_omit_empty_keys():
    assert rustdash.omit({"a": 1, "b": 2}, []) == {"a": 1, "b": 2}

def test_omit_on_non_object():
    assert rustdash.omit("hello", ["a"]) == {}


# ==================== merge Tests ====================

def test_merge_two_objects():
    assert rustdash.merge([{"a": 1}, {"b": 2}]) == {"a": 1, "b": 2}

def test_merge_overlapping_keys_last_wins():
    assert rustdash.merge([{"a": 1}, {"a": 2}]) == {"a": 2}

def test_merge_three_objects():
    assert rustdash.merge([{"a": 1}, {"b": 2}, {"c": 3}]) == {"a": 1, "b": 2, "c": 3}

def test_merge_empty_list():
    assert rustdash.merge([]) == {}

def test_merge_single_object():
    assert rustdash.merge([{"a": 1}]) == {"a": 1}

def test_merge_shallow_no_deep():
    result = rustdash.merge([
        {"config": {"debug": True, "level": 1}},
        {"config": {"debug": False}},
    ])
    # Shallow merge: the entire "config" key is replaced, not deep-merged
    assert result == {"config": {"debug": False}}

def test_merge_mixed_value_types():
    assert rustdash.merge([{"a": 1}, {"a": "hello"}]) == {"a": "hello"}


# ==================== keys Tests ====================

def test_keys_basic():
    result = sorted(rustdash.keys({"a": 1, "b": 2, "c": 3}))
    assert result == ["a", "b", "c"]

def test_keys_empty_object():
    assert rustdash.keys({}) == []

def test_keys_non_object():
    assert rustdash.keys([1, 2, 3]) == []

def test_keys_single_key():
    assert rustdash.keys({"only": True}) == ["only"]


# ==================== values Tests ====================

def test_values_basic():
    result = rustdash.values({"a": 1, "b": 2})
    assert len(result) == 2
    assert 1 in result
    assert 2 in result

def test_values_empty_object():
    assert rustdash.values({}) == []

def test_values_non_object():
    assert rustdash.values(42) == []

def test_values_mixed_types():
    result = rustdash.values({"name": "alice", "age": 30, "active": True})
    assert len(result) == 3
    assert "alice" in result
    assert 30 in result
    assert True in result

def test_values_preserves_nested():
    result = rustdash.values({"data": {"nested": True}})
    assert len(result) == 1
    assert result[0] == {"nested": True}


# ==================== get_all Tests ====================

def test_get_all_wildcard_basic():
    obj = {"users": [{"name": "alice"}, {"name": "bob"}]}
    assert rustdash.get_all(obj, "users[*].name") == ["alice", "bob"]

def test_get_all_wildcard_numbers():
    obj = {"scores": [10, 20, 30]}
    assert rustdash.get_all(obj, "scores[*]") == [10, 20, 30]

def test_get_all_nested_wildcards():
    obj = {"matrix": [[1, 2], [3, 4]]}
    assert rustdash.get_all(obj, "matrix[*][*]") == [1, 2, 3, 4]

def test_get_all_wildcard_deep_path():
    obj = {
        "departments": [
            {"employees": [{"name": "alice"}, {"name": "bob"}]},
            {"employees": [{"name": "charlie"}]},
        ]
    }
    assert rustdash.get_all(obj, "departments[*].employees[*].name") == [
        "alice", "bob", "charlie"
    ]

def test_get_all_missing_field_skipped():
    obj = {
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob"},
        ]
    }
    # bob has no email -> only alice's email is collected
    assert rustdash.get_all(obj, "users[*].email") == ["a@b.com"]

def test_get_all_empty_array():
    assert rustdash.get_all({"users": []}, "users[*].name") == []

def test_get_all_no_wildcard_acts_like_get():
    assert rustdash.get_all({"a": {"b": 1}}, "a.b") == [1]

def test_get_all_path_not_found():
    assert rustdash.get_all({"a": 1}, "z[*].x") == []

def test_get_all_returns_objects():
    obj = {
        "items": [
            {"meta": {"id": 1}},
            {"meta": {"id": 2}},
        ]
    }
    assert rustdash.get_all(obj, "items[*].meta") == [{"id": 1}, {"id": 2}]


# ==================== has_all Tests ====================

def test_has_all_all_present():
    obj = {
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob", "email": "b@c.com"},
        ]
    }
    assert rustdash.has_all(obj, "users[*].email") is True

def test_has_all_some_missing():
    obj = {
        "users": [
            {"name": "alice", "email": "a@b.com"},
            {"name": "bob"},
        ]
    }
    assert rustdash.has_all(obj, "users[*].email") is False

def test_has_all_empty_array_returns_false():
    assert rustdash.has_all({"users": []}, "users[*].name") is False

def test_has_all_nested_wildcards():
    assert rustdash.has_all({"matrix": [[1, 2], [3, 4]]}, "matrix[*][*]") is True

def test_has_all_nested_wildcards_uneven():
    # Second inner array is empty -> no elements to match -> false
    assert rustdash.has_all({"matrix": [[1, 2], []]}, "matrix[*][*]") is False

def test_has_all_no_wildcard_acts_like_has():
    assert rustdash.has_all({"a": {"b": 1}}, "a.b") is True
    assert rustdash.has_all({"a": {"b": 1}}, "a.z") is False

def test_has_all_deep_path():
    obj = {
        "departments": [
            {"employees": [{"name": "alice"}, {"name": "bob"}]},
            {"employees": [{"name": "charlie"}]},
        ]
    }
    assert rustdash.has_all(obj, "departments[*].employees[*].name") is True

def test_has_all_null_values_still_exist():
    obj = {"items": [{"val": None}, {"val": None}]}
    assert rustdash.has_all(obj, "items[*].val") is True
