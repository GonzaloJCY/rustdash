use fancy_regex::Regex;

static RE_CAPITALIZED: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])").unwrap()
});

pub fn split_capitalized_words(words: Vec<&str>) -> Vec<&str> {
    let mut treated_words: Vec<&str> = Vec::new();
    for word in &words {
        let indexes: Vec<usize> = RE_CAPITALIZED
            .find_iter(word)
            .filter_map(|m| m.ok().map(|m| m.start()))
            .collect();

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

    treated_words
}

pub fn clean_delimiters(s: &str) -> Vec<&str> {
    return s
        .split(|c: char| c == '_' || c == '-' || c == ' ')
        .filter(|word| !word.is_empty())
        .collect();
}
