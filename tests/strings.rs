use rustdash::core::strings::{to_parse, StringMode};

#[test]
fn test_camel_case_from_snake_case() {
    assert_eq!(
        to_parse("hello_world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("foo_bar_baz", &StringMode::CamelCase), "fooBarBaz");
    assert_eq!(
        to_parse("test_case_name", &StringMode::CamelCase),
        "testCaseName"
    );
}

#[test]
fn test_camel_case_from_kebab_case() {
    assert_eq!(to_parse("foo-bar-baz", &StringMode::CamelCase), "fooBarBaz");
    assert_eq!(
        to_parse("hello-world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(
        to_parse("my-test-string", &StringMode::CamelCase),
        "myTestString"
    );
}

#[test]
fn test_camel_case_from_space_separated() {
    assert_eq!(
        to_parse("hello world test", &StringMode::CamelCase),
        "helloWorldTest"
    );
}

#[test]
fn test_camel_case_multiple_strings() {
    assert_eq!(
        to_parse("hello_world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("foo-bar", &StringMode::CamelCase), "fooBar");
    assert_eq!(to_parse("test case", &StringMode::CamelCase), "testCase");
}

#[test]
fn test_camel_case_empty_string() {
    assert_eq!(to_parse("", &StringMode::CamelCase), "");
}

#[test]
fn test_camel_case_with_multiple_separators() {
    assert_eq!(
        to_parse("_hello__world_", &StringMode::CamelCase),
        "helloWorld"
    );
}

// ==================== SnakeCase Tests ====================

#[test]
fn test_snake_case_from_camel_case() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::SnakeCase),
        "hello_world"
    );
}

#[test]
fn test_snake_case_from_pascal_case() {
    assert_eq!(
        to_parse("HelloWorld", &StringMode::SnakeCase),
        "hello_world"
    );
}

#[test]
fn test_snake_case_from_kebab_case() {
    assert_eq!(
        to_parse("foo-bar-baz", &StringMode::SnakeCase),
        "foo_bar_baz"
    );
}

#[test]
fn test_snake_case_from_space_separated() {
    assert_eq!(
        to_parse("hello world test", &StringMode::SnakeCase),
        "hello_world_test"
    );
}

#[test]
fn test_snake_case_multiple_strings() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::SnakeCase),
        "hello_world"
    );
    assert_eq!(to_parse("fooBar", &StringMode::SnakeCase), "foo_bar");
}

#[test]
fn test_snake_case_already_snake_case() {
    assert_eq!(
        to_parse("hello_world", &StringMode::SnakeCase),
        "hello_world"
    );
}

#[test]
fn test_snake_case_empty_string() {
    assert_eq!(to_parse("", &StringMode::SnakeCase), "");
}

// ==================== KebabCase Tests ====================

#[test]
fn test_kebab_case_from_camel_case() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::KebabCase),
        "hello-world"
    );
}

#[test]
fn test_kebab_case_from_pascal_case() {
    assert_eq!(
        to_parse("HelloWorld", &StringMode::KebabCase),
        "hello-world"
    );
}

#[test]
fn test_kebab_case_from_snake_case() {
    assert_eq!(
        to_parse("hello_world", &StringMode::KebabCase),
        "hello-world"
    );
}

#[test]
fn test_kebab_case_from_space_separated() {
    assert_eq!(
        to_parse("hello world test", &StringMode::KebabCase),
        "hello-world-test"
    );
}

#[test]
fn test_kebab_case_multiple_strings() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::KebabCase),
        "hello-world"
    );
    assert_eq!(to_parse("fooBar", &StringMode::KebabCase), "foo-bar");
}

#[test]
fn test_kebab_case_already_kebab_case() {
    assert_eq!(
        to_parse("hello-world", &StringMode::KebabCase),
        "hello-world"
    );
}

#[test]
fn test_kebab_case_empty_string() {
    assert_eq!(to_parse("", &StringMode::KebabCase), "");
}

// ==================== PascalCase Tests ====================

#[test]
fn test_pascal_case_from_snake_case() {
    assert_eq!(
        to_parse("hello_world", &StringMode::PascalCase),
        "HelloWorld"
    );
}

#[test]
fn test_pascal_case_from_camel_case() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::PascalCase),
        "HelloWorld"
    );
}

#[test]
fn test_pascal_case_from_kebab_case() {
    assert_eq!(
        to_parse("foo-bar-baz", &StringMode::PascalCase),
        "FooBarBaz"
    );
}

#[test]
fn test_pascal_case_from_space_separated() {
    assert_eq!(
        to_parse("hello world test", &StringMode::PascalCase),
        "HelloWorldTest"
    );
}

#[test]
fn test_pascal_case_multiple_strings() {
    assert_eq!(
        to_parse("hello_world", &StringMode::PascalCase),
        "HelloWorld"
    );
    assert_eq!(to_parse("foo-bar", &StringMode::PascalCase), "FooBar");
    assert_eq!(to_parse("test case", &StringMode::PascalCase), "TestCase");
}

#[test]
fn test_pascal_case_already_pascal_case() {
    assert_eq!(
        to_parse("HelloWorld", &StringMode::PascalCase),
        "HelloWorld"
    );
}

#[test]
fn test_pascal_case_empty_string() {
    assert_eq!(to_parse("", &StringMode::PascalCase), "");
}

#[test]
fn test_pascal_case_with_multiple_separators() {
    assert_eq!(
        to_parse("_hello__world_", &StringMode::PascalCase),
        "HelloWorld"
    );
}

// ==================== Edge Cases ====================

#[test]
fn test_single_word_all_modes() {
    assert_eq!(to_parse("hello", &StringMode::CamelCase), "hello");
    assert_eq!(to_parse("hello", &StringMode::SnakeCase), "hello");
    assert_eq!(to_parse("hello", &StringMode::KebabCase), "hello");
    assert_eq!(to_parse("hello", &StringMode::PascalCase), "Hello");
}

#[test]
fn test_mixed_input_formats() {
    assert_eq!(
        to_parse("hello_world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("fooBar", &StringMode::CamelCase), "fooBar");
    assert_eq!(to_parse("test-case", &StringMode::CamelCase), "testCase");
    assert_eq!(
        to_parse("Another Example", &StringMode::CamelCase),
        "anotherExample"
    );
}

// ==================== Additional Variant Tests ====================

#[test]
fn test_camel_case_variants() {
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::CamelCase),
        "xmlHttpRequest"
    );
    assert_eq!(to_parse("getJSON", &StringMode::CamelCase), "getJson");
    assert_eq!(to_parse("ID", &StringMode::CamelCase), "id");
    assert_eq!(to_parse("user_id_2", &StringMode::CamelCase), "userId2");
    assert_eq!(
        to_parse("api-v2-endpoint", &StringMode::CamelCase),
        "apiV2Endpoint"
    );
}

#[test]
fn test_snake_case_variants() {
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::SnakeCase),
        "xml_http_request"
    );
    assert_eq!(to_parse("getJSON", &StringMode::SnakeCase), "get_json");
    assert_eq!(to_parse("userID", &StringMode::SnakeCase), "user_id");
    assert_eq!(
        to_parse("api-v2-endpoint", &StringMode::SnakeCase),
        "api_v2_endpoint"
    );
    assert_eq!(to_parse("IOError", &StringMode::SnakeCase), "io_error");
}

#[test]
fn test_kebab_case_variants() {
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::KebabCase),
        "xml-http-request"
    );
    assert_eq!(to_parse("getJSON", &StringMode::KebabCase), "get-json");
    assert_eq!(to_parse("userID", &StringMode::KebabCase), "user-id");
    assert_eq!(
        to_parse("hello_world", &StringMode::KebabCase),
        "hello-world"
    );
    assert_eq!(
        to_parse("API Gateway", &StringMode::KebabCase),
        "api-gateway"
    );
}

#[test]
fn test_pascal_case_variants() {
    assert_eq!(
        to_parse("xmlHttpRequest", &StringMode::PascalCase),
        "XmlHttpRequest"
    );
    assert_eq!(to_parse("user_id", &StringMode::PascalCase), "UserId");
    assert_eq!(
        to_parse("api-gateway", &StringMode::PascalCase),
        "ApiGateway"
    );
    assert_eq!(
        to_parse("hello world", &StringMode::PascalCase),
        "HelloWorld"
    );
    assert_eq!(to_parse("io_error", &StringMode::PascalCase), "IoError");
}

#[test]
fn test_numbers_in_strings() {
    // CamelCase with numbers
    assert_eq!(
        to_parse("version_2_api", &StringMode::CamelCase),
        "version2Api"
    );
    assert_eq!(
        to_parse("user123name", &StringMode::CamelCase),
        "user123name"
    );

    // SnakeCase with numbers
    assert_eq!(
        to_parse("version2Api", &StringMode::SnakeCase),
        "version2api"
    );

    // KebabCase with numbers
    assert_eq!(
        to_parse("version2Api", &StringMode::KebabCase),
        "version2api"
    );

    // PascalCase with numbers
    assert_eq!(
        to_parse("version_2_api", &StringMode::PascalCase),
        "Version2Api"
    );
}

#[test]
fn test_single_character_words() {
    assert_eq!(to_parse("a_b_c", &StringMode::CamelCase), "aBC");
    assert_eq!(to_parse("x-y-z", &StringMode::CamelCase), "xYZ");
    assert_eq!(to_parse("ABC", &StringMode::SnakeCase), "abc");
    assert_eq!(to_parse("ABC", &StringMode::KebabCase), "abc");
    assert_eq!(to_parse("a_b_c", &StringMode::PascalCase), "ABC");
}

#[test]
fn test_consecutive_capitals() {
    assert_eq!(
        to_parse("HTTPSConnection", &StringMode::CamelCase),
        "httpsConnection"
    );
    assert_eq!(
        to_parse("HTTPSConnection", &StringMode::SnakeCase),
        "https_connection"
    );
    assert_eq!(
        to_parse("HTTPSConnection", &StringMode::KebabCase),
        "https-connection"
    );
    assert_eq!(
        to_parse("https_connection", &StringMode::PascalCase),
        "HttpsConnection"
    );
}

#[test]
fn test_special_separators() {
    // Multiple consecutive separators
    assert_eq!(
        to_parse("hello___world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("foo---bar", &StringMode::CamelCase), "fooBar");

    // Leading/trailing separators
    assert_eq!(
        to_parse("_hello_world_", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(
        to_parse("-hello-world-", &StringMode::CamelCase),
        "helloWorld"
    );

    // Mixed separators
    assert_eq!(
        to_parse("hello_world-test case", &StringMode::CamelCase),
        "helloWorldTestCase"
    );
}

#[test]
fn test_already_correct_format() {
    // Already camelCase
    assert_eq!(
        to_parse("alreadyCamelCase", &StringMode::CamelCase),
        "alreadyCamelCase"
    );

    // Already snake_case
    assert_eq!(
        to_parse("already_snake_case", &StringMode::SnakeCase),
        "already_snake_case"
    );

    // Already kebab-case
    assert_eq!(
        to_parse("already-kebab-case", &StringMode::KebabCase),
        "already-kebab-case"
    );

    // Already PascalCase
    assert_eq!(
        to_parse("AlreadyPascalCase", &StringMode::PascalCase),
        "AlreadyPascalCase"
    );
}

// ==================== Multiple String Tests ====================

#[test]
fn test_camel_case_multiple_string_list() {
    assert_eq!(
        to_parse("hello_world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("foo-bar-baz", &StringMode::CamelCase), "fooBarBaz");
    assert_eq!(to_parse("test case", &StringMode::CamelCase), "testCase");
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::CamelCase),
        "xmlHttpRequest"
    );
    assert_eq!(to_parse("user_id_2", &StringMode::CamelCase), "userId2");
}

#[test]
fn test_snake_case_multiple_string_list() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::SnakeCase),
        "hello_world"
    );
    assert_eq!(to_parse("fooBarBaz", &StringMode::SnakeCase), "foo_bar_baz");
    assert_eq!(to_parse("TestCase", &StringMode::SnakeCase), "test_case");
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::SnakeCase),
        "xml_http_request"
    );
    assert_eq!(
        to_parse("api-v2-endpoint", &StringMode::SnakeCase),
        "api_v2_endpoint"
    );
}

#[test]
fn test_kebab_case_multiple_string_list() {
    assert_eq!(
        to_parse("helloWorld", &StringMode::KebabCase),
        "hello-world"
    );
    assert_eq!(to_parse("FooBarBaz", &StringMode::KebabCase), "foo-bar-baz");
    assert_eq!(to_parse("test_case", &StringMode::KebabCase), "test-case");
    assert_eq!(
        to_parse("XMLHttpRequest", &StringMode::KebabCase),
        "xml-http-request"
    );
    assert_eq!(to_parse("user ID", &StringMode::KebabCase), "user-id");
}

#[test]
fn test_pascal_case_multiple_string_list() {
    assert_eq!(
        to_parse("hello_world", &StringMode::PascalCase),
        "HelloWorld"
    );
    assert_eq!(to_parse("fooBar", &StringMode::PascalCase), "FooBar");
    assert_eq!(to_parse("test-case", &StringMode::PascalCase), "TestCase");
    assert_eq!(
        to_parse("xml http request", &StringMode::PascalCase),
        "XmlHttpRequest"
    );
    assert_eq!(to_parse("userID", &StringMode::PascalCase), "UserId");
}

#[test]
fn test_large_list_all_modes() {
    // CamelCase
    assert_eq!(to_parse("item_one", &StringMode::CamelCase), "itemOne");
    assert_eq!(to_parse("item-two", &StringMode::CamelCase), "itemTwo");
    assert_eq!(to_parse("ItemThree", &StringMode::CamelCase), "itemThree");
    assert_eq!(to_parse("item four", &StringMode::CamelCase), "itemFour");
    assert_eq!(to_parse("itemFive", &StringMode::CamelCase), "itemFive");
    assert_eq!(to_parse("ITEM_SIX", &StringMode::CamelCase), "itemSix");
    assert_eq!(
        to_parse("item-seven-extra", &StringMode::CamelCase),
        "itemSevenExtra"
    );

    // SnakeCase
    assert_eq!(to_parse("item_one", &StringMode::SnakeCase), "item_one");
    assert_eq!(to_parse("item-two", &StringMode::SnakeCase), "item_two");
    assert_eq!(to_parse("ItemThree", &StringMode::SnakeCase), "item_three");
    assert_eq!(to_parse("item four", &StringMode::SnakeCase), "item_four");
    assert_eq!(to_parse("itemFive", &StringMode::SnakeCase), "item_five");
    assert_eq!(to_parse("ITEM_SIX", &StringMode::SnakeCase), "item_six");
    assert_eq!(
        to_parse("item-seven-extra", &StringMode::SnakeCase),
        "item_seven_extra"
    );

    // KebabCase
    assert_eq!(to_parse("item_one", &StringMode::KebabCase), "item-one");
    assert_eq!(to_parse("item-two", &StringMode::KebabCase), "item-two");
    assert_eq!(to_parse("ItemThree", &StringMode::KebabCase), "item-three");
    assert_eq!(to_parse("item four", &StringMode::KebabCase), "item-four");
    assert_eq!(to_parse("itemFive", &StringMode::KebabCase), "item-five");
    assert_eq!(to_parse("ITEM_SIX", &StringMode::KebabCase), "item-six");
    assert_eq!(
        to_parse("item-seven-extra", &StringMode::KebabCase),
        "item-seven-extra"
    );

    // PascalCase
    assert_eq!(to_parse("item_one", &StringMode::PascalCase), "ItemOne");
    assert_eq!(to_parse("item-two", &StringMode::PascalCase), "ItemTwo");
    assert_eq!(to_parse("ItemThree", &StringMode::PascalCase), "ItemThree");
    assert_eq!(to_parse("item four", &StringMode::PascalCase), "ItemFour");
    assert_eq!(to_parse("itemFive", &StringMode::PascalCase), "ItemFive");
    assert_eq!(to_parse("ITEM_SIX", &StringMode::PascalCase), "ItemSix");
    assert_eq!(
        to_parse("item-seven-extra", &StringMode::PascalCase),
        "ItemSevenExtra"
    );
}

#[test]
fn test_empty_and_whitespace() {
    assert_eq!(
        to_parse("hello_world", &StringMode::CamelCase),
        "helloWorld"
    );
    assert_eq!(to_parse("", &StringMode::CamelCase), "");
    assert_eq!(to_parse("foo-bar", &StringMode::CamelCase), "fooBar");
    assert_eq!(to_parse("   ", &StringMode::CamelCase), "");
    assert_eq!(to_parse("test case", &StringMode::CamelCase), "testCase");

    assert_eq!(
        to_parse("hello_world", &StringMode::SnakeCase),
        "hello_world"
    );
    assert_eq!(to_parse("", &StringMode::SnakeCase), "");
    assert_eq!(to_parse("foo_bar", &StringMode::SnakeCase), "foo_bar");
    assert_eq!(to_parse("   ", &StringMode::SnakeCase), "");
    assert_eq!(to_parse("test case", &StringMode::SnakeCase), "test_case");
}

#[test]
fn test_mixed_formats() {
    // To CamelCase
    assert_eq!(
        to_parse("snake_case_string", &StringMode::CamelCase),
        "snakeCaseString"
    );
    assert_eq!(
        to_parse("kebab-case-string", &StringMode::CamelCase),
        "kebabCaseString"
    );
    assert_eq!(
        to_parse("PascalCaseString", &StringMode::CamelCase),
        "pascalCaseString"
    );
    assert_eq!(
        to_parse("camelCaseString", &StringMode::CamelCase),
        "camelCaseString"
    );
    assert_eq!(
        to_parse("space separated string", &StringMode::CamelCase),
        "spaceSeparatedString"
    );
    assert_eq!(
        to_parse("MixedFormat_with-everything", &StringMode::CamelCase),
        "mixedFormatWithEverything"
    );

    // To SnakeCase
    assert_eq!(
        to_parse("snake_case_string", &StringMode::SnakeCase),
        "snake_case_string"
    );
    assert_eq!(
        to_parse("kebab-case-string", &StringMode::SnakeCase),
        "kebab_case_string"
    );
    assert_eq!(
        to_parse("PascalCaseString", &StringMode::SnakeCase),
        "pascal_case_string"
    );
    assert_eq!(
        to_parse("camelCaseString", &StringMode::SnakeCase),
        "camel_case_string"
    );
    assert_eq!(
        to_parse("space separated string", &StringMode::SnakeCase),
        "space_separated_string"
    );
    assert_eq!(
        to_parse("MixedFormat_with-everything", &StringMode::SnakeCase),
        "mixed_format_with_everything"
    );

    // To KebabCase
    assert_eq!(
        to_parse("snake_case_string", &StringMode::KebabCase),
        "snake-case-string"
    );
    assert_eq!(
        to_parse("kebab-case-string", &StringMode::KebabCase),
        "kebab-case-string"
    );
    assert_eq!(
        to_parse("PascalCaseString", &StringMode::KebabCase),
        "pascal-case-string"
    );
    assert_eq!(
        to_parse("camelCaseString", &StringMode::KebabCase),
        "camel-case-string"
    );
    assert_eq!(
        to_parse("space separated string", &StringMode::KebabCase),
        "space-separated-string"
    );
    assert_eq!(
        to_parse("MixedFormat_with-everything", &StringMode::KebabCase),
        "mixed-format-with-everything"
    );

    // To PascalCase
    assert_eq!(
        to_parse("snake_case_string", &StringMode::PascalCase),
        "SnakeCaseString"
    );
    assert_eq!(
        to_parse("kebab-case-string", &StringMode::PascalCase),
        "KebabCaseString"
    );
    assert_eq!(
        to_parse("PascalCaseString", &StringMode::PascalCase),
        "PascalCaseString"
    );
    assert_eq!(
        to_parse("camelCaseString", &StringMode::PascalCase),
        "CamelCaseString"
    );
    assert_eq!(
        to_parse("space separated string", &StringMode::PascalCase),
        "SpaceSeparatedString"
    );
    assert_eq!(
        to_parse("MixedFormat_with-everything", &StringMode::PascalCase),
        "MixedFormatWithEverything"
    );
}

// ==================== Capitalize Tests ====================

use rustdash::core::strings::{
    _capitalize, _lower_case, _trim, _trim_end, _trim_start, _upper_case, _words,
};

#[test]
fn test_capitalize_basic() {
    assert_eq!(_capitalize("hello"), "Hello");
    assert_eq!(_capitalize("world"), "World");
    assert_eq!(_capitalize("rust"), "Rust");
}

#[test]
fn test_capitalize_already_capitalized() {
    assert_eq!(_capitalize("Hello"), "Hello");
    assert_eq!(_capitalize("HELLO"), "Hello");
}

#[test]
fn test_capitalize_empty_string() {
    assert_eq!(_capitalize(""), "");
}

#[test]
fn test_capitalize_single_char() {
    assert_eq!(_capitalize("a"), "A");
    assert_eq!(_capitalize("Z"), "Z");
}

#[test]
fn test_capitalize_with_spaces() {
    assert_eq!(_capitalize("hello world"), "Hello world");
}

#[test]
fn test_capitalize_numbers() {
    assert_eq!(_capitalize("123abc"), "123abc");
    assert_eq!(_capitalize("a123"), "A123");
}

// ==================== UpperCase Tests ====================

#[test]
fn test_upper_case_basic() {
    assert_eq!(_upper_case("hello"), "HELLO");
    assert_eq!(_upper_case("world"), "WORLD");
    assert_eq!(_upper_case("rust"), "RUST");
}

#[test]
fn test_upper_case_already_upper() {
    assert_eq!(_upper_case("HELLO"), "HELLO");
}

#[test]
fn test_upper_case_mixed() {
    assert_eq!(_upper_case("HeLLo WoRLd"), "HELLO WORLD");
}

#[test]
fn test_upper_case_empty_string() {
    assert_eq!(_upper_case(""), "");
}

#[test]
fn test_upper_case_with_numbers() {
    assert_eq!(_upper_case("hello123"), "HELLO123");
    assert_eq!(_upper_case("test_case"), "TEST_CASE");
}

// ==================== LowerCase Tests ====================

#[test]
fn test_lower_case_basic() {
    assert_eq!(_lower_case("HELLO"), "hello");
    assert_eq!(_lower_case("WORLD"), "world");
    assert_eq!(_lower_case("RUST"), "rust");
}

#[test]
fn test_lower_case_already_lower() {
    assert_eq!(_lower_case("hello"), "hello");
}

#[test]
fn test_lower_case_mixed() {
    assert_eq!(_lower_case("HeLLo WoRLd"), "hello world");
}

#[test]
fn test_lower_case_empty_string() {
    assert_eq!(_lower_case(""), "");
}

#[test]
fn test_lower_case_with_numbers() {
    assert_eq!(_lower_case("HELLO123"), "hello123");
    assert_eq!(_lower_case("TEST_CASE"), "test_case");
}

// ==================== Trim Tests ====================

#[test]
fn test_trim_basic() {
    assert_eq!(_trim("  hello  "), "hello");
    assert_eq!(_trim("   world   "), "world");
}

#[test]
fn test_trim_no_whitespace() {
    assert_eq!(_trim("hello"), "hello");
}

#[test]
fn test_trim_only_whitespace() {
    assert_eq!(_trim("   "), "");
    assert_eq!(_trim("\t\n"), "");
}

#[test]
fn test_trim_empty_string() {
    assert_eq!(_trim(""), "");
}

#[test]
fn test_trim_tabs_and_newlines() {
    assert_eq!(_trim("\thello\n"), "hello");
    assert_eq!(_trim("\n\t  hello world  \t\n"), "hello world");
}

#[test]
fn test_trim_internal_spaces_preserved() {
    assert_eq!(_trim("  hello world  "), "hello world");
}

// ==================== TrimStart Tests ====================

#[test]
fn test_trim_start_basic() {
    assert_eq!(_trim_start("  hello"), "hello");
    assert_eq!(_trim_start("   world"), "world");
}

#[test]
fn test_trim_start_trailing_preserved() {
    assert_eq!(_trim_start("  hello  "), "hello  ");
}

#[test]
fn test_trim_start_no_leading_whitespace() {
    assert_eq!(_trim_start("hello  "), "hello  ");
}

#[test]
fn test_trim_start_empty_string() {
    assert_eq!(_trim_start(""), "");
}

#[test]
fn test_trim_start_tabs_and_newlines() {
    assert_eq!(_trim_start("\t\nhello"), "hello");
}

// ==================== TrimEnd Tests ====================

#[test]
fn test_trim_end_basic() {
    assert_eq!(_trim_end("hello  "), "hello");
    assert_eq!(_trim_end("world   "), "world");
}

#[test]
fn test_trim_end_leading_preserved() {
    assert_eq!(_trim_end("  hello  "), "  hello");
}

#[test]
fn test_trim_end_no_trailing_whitespace() {
    assert_eq!(_trim_end("  hello"), "  hello");
}

#[test]
fn test_trim_end_empty_string() {
    assert_eq!(_trim_end(""), "");
}

#[test]
fn test_trim_end_tabs_and_newlines() {
    assert_eq!(_trim_end("hello\t\n"), "hello");
}

// ==================== Words Tests ====================

#[test]
fn test_words_basic() {
    assert_eq!(_words("hello world foo"), vec!["hello", "world", "foo"]);
}

#[test]
fn test_words_single_word() {
    assert_eq!(_words("hello"), vec!["hello"]);
}

#[test]
fn test_words_multiple_spaces() {
    assert_eq!(_words("hello   world"), vec!["hello", "world"]);
}

#[test]
fn test_words_leading_trailing_spaces() {
    assert_eq!(_words("  hello world  "), vec!["hello", "world"]);
}

#[test]
fn test_words_empty_string() {
    let empty: Vec<&str> = vec![];
    assert_eq!(_words(""), empty);
}

#[test]
fn test_words_only_whitespace() {
    let empty: Vec<&str> = vec![];
    assert_eq!(_words("   "), empty);
}

#[test]
fn test_words_tabs_and_newlines() {
    assert_eq!(_words("hello\tworld\nfoo"), vec!["hello", "world", "foo"]);
}

#[test]
fn test_words_mixed_whitespace() {
    assert_eq!(
        _words("  hello \t world \n foo  "),
        vec!["hello", "world", "foo"]
    );
}
