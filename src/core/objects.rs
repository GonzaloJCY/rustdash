/// Parse a dotted/bracket path into tokens.
///
/// Examples:
///   "a.b.c"           → [Key("a"), Key("b"), Key("c")]
///   "items[0]"        → [Key("items"), Index(0)]
///   "users[0].name"   → [Key("users"), Index(0), Key("name")]
///   "matrix[0][1]"    → [Key("matrix"), Index(0), Index(1)]
///   "users[*].name"   → [Key("users"), Wildcard, Key("name")]
pub fn parse_path(path: &str) -> Vec<Token> {
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

// ─── Path tokenizer (shared by get, has, get_all, has_all) ──────────────────

pub enum Token {
    Key(String),
    Index(usize),
    Wildcard,
}
