use crate::core::strings_helpers::clean_delimiters;
use crate::core::strings_helpers::split_capitalized_words;

pub enum StringMode {
    CamelCase,
    SnakeCase,
    KebabCase,
    PascalCase,
}

/// Converts strings to the selected mode. Handles single string or Vec<String>.
pub fn to_parse(input: Vec<String>, string_mode: &StringMode) -> Vec<String> {
    input
        .into_iter()
        .map(|s| to_parse_single(&s, string_mode))
        .collect()
}

/// Converts single string to camelCase
pub fn to_parse_single(s: &str, string_mode: &StringMode) -> String {
    if s.is_empty() {
        return String::new();
    }
    let words: Vec<&str> = clean_delimiters(s);

    let treated_words: Vec<&str> = split_capitalized_words(words);

    match string_mode {
        StringMode::CamelCase => treated_words
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut chars = s.chars();
                match chars.next() {
                    Some(first) => {
                        if i == 0 {
                            first.to_lowercase().to_string() + &chars.as_str().to_lowercase()
                        } else {
                            first.to_uppercase().to_string() + &chars.as_str().to_lowercase()
                        }
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<String>>()
            .join(""),

        StringMode::SnakeCase => treated_words.join("_").to_lowercase(),
        StringMode::KebabCase => treated_words.join("-").to_lowercase(),
        StringMode::PascalCase => treated_words
            .iter()
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    Some(first) => {
                        first.to_uppercase().to_string() + &chars.as_str().to_lowercase()
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<String>>()
            .join(""),
    }
}

pub fn _capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

pub fn _upper_case(s: &str) -> String {
    s.to_uppercase()
}

pub fn _lower_case(s: &str) -> String {
    s.to_lowercase()
}

pub fn _trim(s: &str) -> String {
    s.trim().to_string()
}

pub fn _trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

pub fn _trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

pub fn _words(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}