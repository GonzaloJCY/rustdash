use serde_json::Value;
/// Parse a dotted/bracket path into tokens.
///
/// Examples:
///   "a.b.c"           → [Key("a"), Key("b"), Key("c")]
///   "items[0]"        → [Key("items"), Index(0)]
///   "users[0].name"   → [Key("users"), Index(0), Key("name")]
///   "matrix[0][1]"    → [Key("matrix"), Index(0), Index(1)]
///   "users[*].name"   → [Key("users"), Wildcard, Key("name")]
fn parse_path(path: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // Split on '.' first, then handle brackets within each segment.
    for segment in path.split('.') {
        // Check if this segment contains bracket notation.
        if let Some(bracket_start) = segment.find('[') {
            // Part before the first '[' is a key (if non-empty).
            let key_part = &segment[..bracket_start];
            if !key_part.is_empty() {
                tokens.push(Token::Key(key_part.to_string()));
            }
            // Process all bracket pairs: "[0]", "[*]", "[12]", etc.
            let mut rest = &segment[bracket_start..];
            while rest.starts_with('[') {
                if let Some(close) = rest.find(']') {
                    let inner = &rest[1..close];
                    if inner == "*" {
                        tokens.push(Token::Wildcard);
                    } else if let Ok(idx) = inner.parse::<usize>() {
                        tokens.push(Token::Index(idx));
                    }
                    rest = &rest[close + 1..];
                } else {
                    break;
                }
            }
        } else {
            // Plain key, no brackets.
            tokens.push(Token::Key(segment.to_string()));
        }
    }
    tokens
}

fn resolve_path(current: &Value, tokens: &[Token]) -> Vec<Value> {
    // Base case: no tokens left = success, return current value
    if tokens.is_empty() {
        return vec![current.clone()];
    }

    let first = &tokens[0];
    let rest = &tokens[1..];

    match first {
        Token::Key(key) => {
            // Try exact key match, continue recursion
            if let Some(obj) = current.as_object() {
                if let Some(next) = obj.get(key) {
                    return resolve_path(next, rest);
                }
            }
            // Key missing = fail this path
            vec![]
        }
        Token::Index(idx) => {
            // Try exact index, continue recursion
            if let Some(arr) = current.as_array() {
                if *idx < arr.len() {
                    let next = &arr[*idx];
                    return resolve_path(next, rest);
                }
            }
            vec![]
        }
        Token::Wildcard => {
            // CRITICAL: Explore ALL array elements independently
            let mut all_results = Vec::new();
            if let Some(arr) = current.as_array() {
                for item in arr {
                    // Each array item gets full remaining path
                    let path_results = resolve_path(item, rest);
                    all_results.extend(path_results);
                }
            }
            all_results
        }
    }
}

fn has_all_paths(current: &Value, tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return true; // Leaf reached = path exists
    }

    let first = &tokens[0];
    let rest = &tokens[1..];

    match first {
        Token::Key(key) => {
            if let Some(obj) = current.as_object() {
                obj.get(key).map_or(false, |next| has_all_paths(next, rest))
            } else {
                false
            }
        }
        Token::Index(idx) => {
            if let Some(arr) = current.as_array() {
                if *idx < arr.len() {
                    has_all_paths(&arr[*idx], rest)
                } else {
                    false
                }
            } else {
                false
            }
        }
        Token::Wildcard => {
            if let Some(arr) = current.as_array() {
                if arr.is_empty() {
                    return false; // Empty array = no matches
                }
                // EVERY array element must have the remaining path
                arr.iter().all(|item| has_all_paths(item, rest))
            } else {
                false
            }
        }
    }
}
pub fn get(obj: &Value, path: &str, default: Value) -> Value {
    // Step 1: Tokenize the path into a flat list of tokens.
    // "users[0].name" → [Key("users"), Index(0), Key("name")]
    let tokens = parse_path(path);

    // Step 2: Walk the tokens, drilling into `obj` one level at a time.
    let mut current = obj;
    for token in &tokens {
        match token {
            Token::Key(key) => {
                // Current value must be an object; look up the key.
                match current.get(key.as_str()) {
                    Some(next) => current = next,
                    None => return default,
                }
            }
            Token::Index(i) => {
                // Current value must be an array; index into it.
                match current.get(*i) {
                    Some(next) => current = next,
                    None => return default,
                }
            }
            Token::Wildcard => return default, // wildcards are for get_all, not get
        }
    }
    current.clone()
}

// ─── Path tokenizer (shared by get, has, get_all, has_all) ──────────────────

enum Token {
    Key(String),
    Index(usize),
    Wildcard,
}

/// Returns `true` when `path` resolves to an existing (non-missing) entry
/// inside `obj`.  Uses the same path syntax as [`get`].
pub fn has(obj: &Value, path: &str) -> bool {
    // Step 1: Tokenize the path into a flat list of tokens.
    // "users[0].name" → [Key("users"), Index(0), Key("name")]
    let tokens = parse_path(path);

    // Step 2: Walk the tokens, drilling into `obj` one level at a time.
    let mut current = obj;
    for token in &tokens {
        match token {
            Token::Key(key) => {
                // Current value must be an object; look up the key.
                match current.get(key.as_str()) {
                    Some(next) => current = next,
                    None => return false,
                }
            }
            Token::Index(i) => {
                // Current value must be an array; index into it.
                match current.get(*i) {
                    Some(next) => current = next,
                    None => return false,
                }
            }
            Token::Wildcard => return false, // wildcards are for get_all, not get
        }
    }
    true
}

/// Collect **all** values that match a wildcard path containing `[*]`.
/// `[*]` expands to every element of an array.
///
/// Example: `get_all(obj, "users[*].name")` returns a `Vec<Value>` with
/// every user's name.  Multiple wildcards are supported
/// (e.g. `"matrix[*][*]"`).  Returns an empty vec when nothing matches.
pub fn get_all(obj: &Value, pattern: &str) -> Vec<Value> {
    let tokens = parse_path(pattern);
    resolve_path(obj, &tokens)
}

/// Returns `true` only when **every** expansion of the `[*]` wildcard in
/// `pattern` resolves to an existing entry.
///
/// Example: `has_all(obj, "users[*].email")` is `true` only if every
/// element in the `users` array has an `email` field.  Returns `false`
/// for empty arrays (nothing to match).
pub fn has_all(obj: &Value, pattern: &str) -> bool {
    let tokens = parse_path(pattern);
    has_all_paths(obj, &tokens)
}

/// Return a new JSON object that keeps only the listed top-level `keys`
/// from `obj`.  Keys that do not exist are silently ignored.
pub fn pick(obj: &Value, keys: &[&str]) -> Value {
    if let Some(input_obj) = obj.as_object() {
        let mut result = serde_json::Map::new();

        for key in keys {
            if let Some(value) = input_obj.get(*key) {
                result.insert(key.to_string(), value.clone());
            }
        }

        Value::Object(result)
    } else {
        // Non-objects return empty object (Lodash behavior)
        Value::Object(serde_json::Map::new())
    }
}
/// Return a new JSON object that contains every top-level key from `obj`
/// **except** those listed in `keys`.
pub fn omit(obj: &Value, keys: &[&str]) -> Value {
    if let Some(input_obj) = obj.as_object() {
        let mut result = serde_json::Map::new();

        // Copy ALL keys first
        for (key, value) in input_obj.iter() {
            result.insert(key.clone(), value.clone());
        }

        // Remove specified keys
        for key in keys {
            result.remove(*key);
        }

        Value::Object(result)
    } else {
        // Non-objects return empty object (Lodash behavior)
        Value::Object(serde_json::Map::new())
    }
}
/// Shallow-merge an ordered slice of JSON objects.  Later objects overwrite
/// keys from earlier ones ("last wins").
pub fn merge(objects: &[Value]) -> Value {
    let mut result = serde_json::Map::new();

    for obj in objects {
        if let Some(obj_map) = obj.as_object() {
            // Later objects overwrite earlier ones
            for (key, value) in obj_map {
                result.insert(key.clone(), value.clone());
            }
        }
        // Non-objects silently ignored (Lodash behavior)
    }

    Value::Object(result)
}

/// Return the top-level keys of a JSON object (empty vec for non-objects).
pub fn keys(obj: &Value) -> Vec<String> {
    if let Some(obj_map) = obj.as_object() {
        obj_map.keys().cloned().collect()
    } else {
        vec![]
    }
}

/// Return the top-level values of a JSON object (empty vec for non-objects).
pub fn values(obj: &Value) -> Vec<Value> {
    if let Some(obj_map) = obj.as_object() {
        obj_map.values().cloned().collect()
    } else {
        vec![]
    }
}
