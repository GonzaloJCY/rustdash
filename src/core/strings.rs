use fancy_regex::Regex;

pub enum StringMode {
    CamelCase,
    SnakeCase,
    KebabCase,
    PascalCase,
}

/// Converts strings to the selected mode. Handles single string or Vec<String>.
pub fn to_parse(input: Vec<String>, string_mode: StringMode) -> Vec<String> {
    input
        .into_iter()
        .map(|s| to_parse_single(&s, &string_mode))
        .collect()
}

/// Converts single string to camelCase
fn to_parse_single(s: &str, string_mode: &StringMode) -> String {
    if s.is_empty() {
        return String::new();
    }
    let re = Regex::new(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])").unwrap();

    match string_mode {
        StringMode::CamelCase => {
            let mut result = String::new();

            if s.is_empty() {
                return result;
            }
            let mut treated_words: Vec<&str> = Vec::new();

            let words: Vec<&str> = s
                .split(|c: char| c == '_' || c == '-' || c == ' ')
                .filter(|word| !word.is_empty())
                .collect();


            for word in &words {
                let indexes: Vec<usize> = re.find_iter(word).filter_map(|m| m.ok().map(|m| m.start())).collect();

                if !indexes.is_empty() {
                    let mut previous_index = 0;

                    for index in indexes {
                        if previous_index != index {
                            treated_words.push(&word[previous_index..index]);
                            previous_index = index;
                        }
                    }
                    treated_words.push(&word[previous_index..]);
                } else {
                    treated_words.push(word);
                }
            }



            for (i, slice) in treated_words.iter().enumerate(){

                let mut  chars = slice.chars();
                if let Some(first) =chars.next() {
                    if i == 0 {
                        result.push_str(&first.to_lowercase().to_string());
                        result.push_str(&chars.as_str().to_lowercase());
                        continue;
                        
                    } else {
                        result.push_str(&first.to_uppercase().to_string());
                        result.push_str(&chars.as_str().to_lowercase());
                    }

        
                    
                }

            }
            result
        }
        StringMode::SnakeCase => {
            let mut result = String::new();

            if s.is_empty() {
                return result;
            }
            let mut treated_words: Vec<&str> = Vec::new();

            let words: Vec<&str> = s
                .split(|c: char| c == '_' || c == '-' || c == ' ')
                .filter(|word| !word.is_empty())
                .collect();


            for word in &words {
                let indexes: Vec<usize> = re.find_iter(word).filter_map(|m| m.ok().map(|m| m.start())).collect();

                if !indexes.is_empty() {
                    let mut previous_index = 0;

                    for index in indexes {
                        if previous_index != index {
                            treated_words.push(&word[previous_index..index]);
                            previous_index = index;
                        }
                    }
                    treated_words.push(&word[previous_index..]);
                } else {
                    treated_words.push(word);
                }
            }

            let mut counter = 0;

            for slice in &treated_words {
                result.push_str(&slice.to_lowercase());
                if counter < treated_words.len() - 1 {
                    result.push('_');
                    counter += 1;
                }
            }
            result
        }
        StringMode::KebabCase => {
            let mut result = String::new();

            if s.is_empty() {
                return result;
            }
            let mut treated_words: Vec<&str> = Vec::new();

            let words: Vec<&str> = s
                .split(|c: char| c == '_' || c == '-' || c == ' ')
                .filter(|word| !word.is_empty())
                .collect();


            for word in &words {
                let indexes: Vec<usize> = re.find_iter(word).filter_map(|m| m.ok().map(|m| m.start())).collect();

                if !indexes.is_empty() {
                    let mut previous_index = 0;

                    for index in indexes {
                        if previous_index != index {
                            treated_words.push(&word[previous_index..index]);
                            previous_index = index;
                        }
                    }
                    treated_words.push(&word[previous_index..]);
                } else {
                    treated_words.push(word);
                }
            }

            let mut counter = 0;

            for slice in &treated_words {
                result.push_str(&slice.to_lowercase());
                if counter < treated_words.len() - 1 {
                    result.push('-');
                    counter += 1;
                }
            }
            result
        }
        StringMode::PascalCase => {
            let mut result = String::new();

            if s.is_empty() {
                return result;
            }
            let mut treated_words: Vec<&str> = Vec::new();

            let words: Vec<&str> = s
                .split(|c: char| c == '_' || c == '-' || c == ' ')
                .filter(|word| !word.is_empty())
                .collect();


            for word in &words {
                let indexes: Vec<usize> = re.find_iter(word).filter_map(|m| m.ok().map(|m| m.start())).collect();

                if !indexes.is_empty() {
                    let mut previous_index = 0;

                    for index in indexes {
                        if previous_index != index {
                            treated_words.push(&word[previous_index..index]);
                            previous_index = index;
                        }
                    }
                    treated_words.push(&word[previous_index..]);
                } else {
                    treated_words.push(word);
                }
            }


            for slice in &treated_words {
                let mut  chars = slice.chars();
                if let Some(first) =chars.next() {
                    result.push_str(&first.to_uppercase().to_string());
                    result.push_str(&chars.as_str().to_lowercase());
                    
                    
                }

            }
            result
        }
    }
}
