use rustdash::core::strings::{to_parse, StringMode};

#[test]
fn test_camel_case_from_snake_case() {
    assert_eq!(
        to_parse(vec!["hello_world".to_string()], StringMode::CamelCase),
        vec!["helloWorld".to_string()]
    );
    assert_eq!(
        to_parse(vec!["foo_bar_baz".to_string()], StringMode::CamelCase),
        vec!["fooBarBaz".to_string()]
    );
    assert_eq!(
        to_parse(vec!["test_case_name".to_string()], StringMode::CamelCase),
        vec!["testCaseName".to_string()]
    );
}

#[test]
fn test_camel_case_from_kebab_case() {
    assert_eq!(
        to_parse(vec!["foo-bar-baz".to_string()], StringMode::CamelCase),
        vec!["fooBarBaz".to_string()]
    );
    assert_eq!(
        to_parse(vec!["hello-world".to_string()], StringMode::CamelCase),
        vec!["helloWorld".to_string()]
    );
    assert_eq!(
        to_parse(vec!["my-test-string".to_string()], StringMode::CamelCase),
        vec!["myTestString".to_string()]
    );
}

#[test]
fn test_camel_case_from_space_separated() {
    let input = vec!["hello world test".to_string()];
    let expected = vec!["helloWorldTest".to_string()];
    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

#[test]
fn test_camel_case_multiple_strings() {
    let input = vec![
        "hello_world".to_string(),
        "foo-bar".to_string(),
        "test case".to_string(),
    ];
    let expected = vec![
        "helloWorld".to_string(),
        "fooBar".to_string(),
        "testCase".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

#[test]
fn test_camel_case_empty_string() {
    let input = vec!["".to_string()];
    let expected = vec!["".to_string()];
    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

#[test]
fn test_camel_case_with_multiple_separators() {
    let input = vec!["_hello__world_".to_string()];
    let expected = vec!["helloWorld".to_string()];
    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

// ==================== SnakeCase Tests ====================

#[test]
fn test_snake_case_from_camel_case() {
    let input = vec!["helloWorld".to_string()];
    let expected = vec!["hello_world".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_from_pascal_case() {
    let input = vec!["HelloWorld".to_string()];
    let expected = vec!["hello_world".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_from_kebab_case() {
    let input = vec!["foo-bar-baz".to_string()];
    let expected = vec!["foo_bar_baz".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_from_space_separated() {
    let input = vec!["hello world test".to_string()];
    let expected = vec!["hello_world_test".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_multiple_strings() {
    let input = vec!["helloWorld".to_string(), "fooBar".to_string()];
    let expected = vec!["hello_world".to_string(), "foo_bar".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_already_snake_case() {
    let input = vec!["hello_world".to_string()];
    let expected = vec!["hello_world".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_snake_case_empty_string() {
    let input = vec!["".to_string()];
    let expected = vec!["".to_string()];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

// ==================== KebabCase Tests ====================

#[test]
fn test_kebab_case_from_camel_case() {
    let input = vec!["helloWorld".to_string()];
    let expected = vec!["hello-world".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_from_pascal_case() {
    let input = vec!["HelloWorld".to_string()];
    let expected = vec!["hello-world".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_from_snake_case() {
    let input = vec!["hello_world".to_string()];
    let expected = vec!["hello-world".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_from_space_separated() {
    let input = vec!["hello world test".to_string()];
    let expected = vec!["hello-world-test".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_multiple_strings() {
    let input = vec!["helloWorld".to_string(), "fooBar".to_string()];
    let expected = vec!["hello-world".to_string(), "foo-bar".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_already_kebab_case() {
    let input = vec!["hello-world".to_string()];
    let expected = vec!["hello-world".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_kebab_case_empty_string() {
    let input = vec!["".to_string()];
    let expected = vec!["".to_string()];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

// ==================== PascalCase Tests ====================

#[test]
fn test_pascal_case_from_snake_case() {
    let input = vec!["hello_world".to_string()];
    let expected = vec!["HelloWorld".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_from_camel_case() {
    let input = vec!["helloWorld".to_string()];
    let expected = vec!["HelloWorld".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_from_kebab_case() {
    let input = vec!["foo-bar-baz".to_string()];
    let expected = vec!["FooBarBaz".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_from_space_separated() {
    let input = vec!["hello world test".to_string()];
    let expected = vec!["HelloWorldTest".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_multiple_strings() {
    let input = vec![
        "hello_world".to_string(),
        "foo-bar".to_string(),
        "test case".to_string(),
    ];
    let expected = vec![
        "HelloWorld".to_string(),
        "FooBar".to_string(),
        "TestCase".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_already_pascal_case() {
    let input = vec!["HelloWorld".to_string()];
    let expected = vec!["HelloWorld".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_empty_string() {
    let input = vec!["".to_string()];
    let expected = vec!["".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_pascal_case_with_multiple_separators() {
    let input = vec!["_hello__world_".to_string()];
    let expected = vec!["HelloWorld".to_string()];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

// ==================== Edge Cases ====================

#[test]
fn test_single_word_all_modes() {
    let input = vec!["hello".to_string()];

    assert_eq!(
        to_parse(input.clone(), StringMode::CamelCase),
        vec!["hello".to_string()]
    );
    assert_eq!(
        to_parse(input.clone(), StringMode::SnakeCase),
        vec!["hello".to_string()]
    );
    assert_eq!(
        to_parse(input.clone(), StringMode::KebabCase),
        vec!["hello".to_string()]
    );
    assert_eq!(
        to_parse(input.clone(), StringMode::PascalCase),
        vec!["Hello".to_string()]
    );
}

#[test]
fn test_mixed_input_formats() {
    let input = vec![
        "hello_world".to_string(),
        "fooBar".to_string(),
        "test-case".to_string(),
        "Another Example".to_string(),
    ];

    let expected = vec![
        "helloWorld".to_string(),
        "fooBar".to_string(),
        "testCase".to_string(),
        "anotherExample".to_string(),
    ];

    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

// ==================== Additional Variant Tests ====================

#[test]
fn test_camel_case_variants() {
    assert_eq!(
        to_parse(vec!["XMLHttpRequest".to_string()], StringMode::CamelCase),
        vec!["xmlHttpRequest".to_string()]
    );
    assert_eq!(
        to_parse(vec!["getJSON".to_string()], StringMode::CamelCase),
        vec!["getJson".to_string()]
    );
    assert_eq!(
        to_parse(vec!["ID".to_string()], StringMode::CamelCase),
        vec!["id".to_string()]
    );
    assert_eq!(
        to_parse(vec!["user_id_2".to_string()], StringMode::CamelCase),
        vec!["userId2".to_string()]
    );
    assert_eq!(
        to_parse(vec!["api-v2-endpoint".to_string()], StringMode::CamelCase),
        vec!["apiV2Endpoint".to_string()]
    );
}

#[test]
fn test_snake_case_variants() {
    assert_eq!(
        to_parse(vec!["XMLHttpRequest".to_string()], StringMode::SnakeCase),
        vec!["xml_http_request".to_string()]
    );
    assert_eq!(
        to_parse(vec!["getJSON".to_string()], StringMode::SnakeCase),
        vec!["get_json".to_string()]
    );
    assert_eq!(
        to_parse(vec!["userID".to_string()], StringMode::SnakeCase),
        vec!["user_id".to_string()]
    );
    assert_eq!(
        to_parse(vec!["api-v2-endpoint".to_string()], StringMode::SnakeCase),
        vec!["api_v2_endpoint".to_string()]
    );
    assert_eq!(
        to_parse(vec!["IOError".to_string()], StringMode::SnakeCase),
        vec!["io_error".to_string()]
    );
}

#[test]
fn test_kebab_case_variants() {
    assert_eq!(
        to_parse(vec!["XMLHttpRequest".to_string()], StringMode::KebabCase),
        vec!["xml-http-request".to_string()]
    );
    assert_eq!(
        to_parse(vec!["getJSON".to_string()], StringMode::KebabCase),
        vec!["get-json".to_string()]
    );
    assert_eq!(
        to_parse(vec!["userID".to_string()], StringMode::KebabCase),
        vec!["user-id".to_string()]
    );
    assert_eq!(
        to_parse(vec!["hello_world".to_string()], StringMode::KebabCase),
        vec!["hello-world".to_string()]
    );
    assert_eq!(
        to_parse(vec!["API Gateway".to_string()], StringMode::KebabCase),
        vec!["api-gateway".to_string()]
    );
}

#[test]
fn test_pascal_case_variants() {
    assert_eq!(
        to_parse(vec!["xmlHttpRequest".to_string()], StringMode::PascalCase),
        vec!["XmlHttpRequest".to_string()]
    );
    assert_eq!(
        to_parse(vec!["user_id".to_string()], StringMode::PascalCase),
        vec!["UserId".to_string()]
    );
    assert_eq!(
        to_parse(vec!["api-gateway".to_string()], StringMode::PascalCase),
        vec!["ApiGateway".to_string()]
    );
    assert_eq!(
        to_parse(vec!["hello world".to_string()], StringMode::PascalCase),
        vec!["HelloWorld".to_string()]
    );
    assert_eq!(
        to_parse(vec!["io_error".to_string()], StringMode::PascalCase),
        vec!["IoError".to_string()]
    );
}

#[test]
fn test_numbers_in_strings() {
    // CamelCase with numbers
    assert_eq!(
        to_parse(vec!["version_2_api".to_string()], StringMode::CamelCase),
        vec!["version2Api".to_string()]
    );
    assert_eq!(
        to_parse(vec!["user123name".to_string()], StringMode::CamelCase),
        vec!["user123name".to_string()]
    );

    // SnakeCase with numbers
    assert_eq!(
        to_parse(vec!["version2Api".to_string()], StringMode::SnakeCase),
        vec!["version2api".to_string()]
    );

    // KebabCase with numbers
    assert_eq!(
        to_parse(vec!["version2Api".to_string()], StringMode::KebabCase),
        vec!["version2api".to_string()]
    );

    // PascalCase with numbers
    assert_eq!(
        to_parse(vec!["version_2_api".to_string()], StringMode::PascalCase),
        vec!["Version2Api".to_string()]
    );
}

#[test]
fn test_single_character_words() {
    assert_eq!(
        to_parse(vec!["a_b_c".to_string()], StringMode::CamelCase),
        vec!["aBC".to_string()]
    );
    assert_eq!(
        to_parse(vec!["x-y-z".to_string()], StringMode::CamelCase),
        vec!["xYZ".to_string()]
    );
    assert_eq!(
        to_parse(vec!["ABC".to_string()], StringMode::SnakeCase),
        vec!["abc".to_string()]
    );
    assert_eq!(
        to_parse(vec!["ABC".to_string()], StringMode::KebabCase),
        vec!["abc".to_string()]
    );
    assert_eq!(
        to_parse(vec!["a_b_c".to_string()], StringMode::PascalCase),
        vec!["ABC".to_string()]
    );
}

#[test]
fn test_consecutive_capitals() {
    assert_eq!(
        to_parse(vec!["HTTPSConnection".to_string()], StringMode::CamelCase),
        vec!["httpsConnection".to_string()]
    );
    assert_eq!(
        to_parse(vec!["HTTPSConnection".to_string()], StringMode::SnakeCase),
        vec!["https_connection".to_string()]
    );
    assert_eq!(
        to_parse(vec!["HTTPSConnection".to_string()], StringMode::KebabCase),
        vec!["https-connection".to_string()]
    );
    assert_eq!(
        to_parse(vec!["https_connection".to_string()], StringMode::PascalCase),
        vec!["HttpsConnection".to_string()]
    );
}

#[test]
fn test_special_separators() {
    // Multiple consecutive separators
    assert_eq!(
        to_parse(vec!["hello___world".to_string()], StringMode::CamelCase),
        vec!["helloWorld".to_string()]
    );
    assert_eq!(
        to_parse(vec!["foo---bar".to_string()], StringMode::CamelCase),
        vec!["fooBar".to_string()]
    );

    // Leading/trailing separators
    assert_eq!(
        to_parse(vec!["_hello_world_".to_string()], StringMode::CamelCase),
        vec!["helloWorld".to_string()]
    );
    assert_eq!(
        to_parse(vec!["-hello-world-".to_string()], StringMode::CamelCase),
        vec!["helloWorld".to_string()]
    );

    // Mixed separators
    assert_eq!(
        to_parse(
            vec!["hello_world-test case".to_string()],
            StringMode::CamelCase
        ),
        vec!["helloWorldTestCase".to_string()]
    );
}

#[test]
fn test_already_correct_format() {
    // Already camelCase
    assert_eq!(
        to_parse(vec!["alreadyCamelCase".to_string()], StringMode::CamelCase),
        vec!["alreadyCamelCase".to_string()]
    );

    // Already snake_case
    assert_eq!(
        to_parse(
            vec!["already_snake_case".to_string()],
            StringMode::SnakeCase
        ),
        vec!["already_snake_case".to_string()]
    );

    // Already kebab-case
    assert_eq!(
        to_parse(
            vec!["already-kebab-case".to_string()],
            StringMode::KebabCase
        ),
        vec!["already-kebab-case".to_string()]
    );

    // Already PascalCase
    assert_eq!(
        to_parse(
            vec!["AlreadyPascalCase".to_string()],
            StringMode::PascalCase
        ),
        vec!["AlreadyPascalCase".to_string()]
    );
}

// ==================== Multiple String List Tests ====================

#[test]
fn test_camel_case_multiple_string_list() {
    let input = vec![
        "hello_world".to_string(),
        "foo-bar-baz".to_string(),
        "test case".to_string(),
        "XMLHttpRequest".to_string(),
        "user_id_2".to_string(),
    ];
    let expected = vec![
        "helloWorld".to_string(),
        "fooBarBaz".to_string(),
        "testCase".to_string(),
        "xmlHttpRequest".to_string(),
        "userId2".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::CamelCase), expected);
}

#[test]
fn test_snake_case_multiple_string_list() {
    let input = vec![
        "helloWorld".to_string(),
        "fooBarBaz".to_string(),
        "TestCase".to_string(),
        "XMLHttpRequest".to_string(),
        "api-v2-endpoint".to_string(),
    ];
    let expected = vec![
        "hello_world".to_string(),
        "foo_bar_baz".to_string(),
        "test_case".to_string(),
        "xml_http_request".to_string(),
        "api_v2_endpoint".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::SnakeCase), expected);
}

#[test]
fn test_kebab_case_multiple_string_list() {
    let input = vec![
        "helloWorld".to_string(),
        "FooBarBaz".to_string(),
        "test_case".to_string(),
        "XMLHttpRequest".to_string(),
        "user ID".to_string(),
    ];
    let expected = vec![
        "hello-world".to_string(),
        "foo-bar-baz".to_string(),
        "test-case".to_string(),
        "xml-http-request".to_string(),
        "user-id".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::KebabCase), expected);
}

#[test]
fn test_pascal_case_multiple_string_list() {
    let input = vec![
        "hello_world".to_string(),
        "fooBar".to_string(),
        "test-case".to_string(),
        "xml http request".to_string(),
        "userID".to_string(),
    ];
    let expected = vec![
        "HelloWorld".to_string(),
        "FooBar".to_string(),
        "TestCase".to_string(),
        "XmlHttpRequest".to_string(),
        "UserId".to_string(),
    ];
    assert_eq!(to_parse(input, StringMode::PascalCase), expected);
}

#[test]
fn test_large_list_all_modes() {
    let input = vec![
        "item_one".to_string(),
        "item-two".to_string(),
        "ItemThree".to_string(),
        "item four".to_string(),
        "itemFive".to_string(),
        "ITEM_SIX".to_string(),
        "item-seven-extra".to_string(),
    ];

    // CamelCase
    let camel_expected = vec![
        "itemOne".to_string(),
        "itemTwo".to_string(),
        "itemThree".to_string(),
        "itemFour".to_string(),
        "itemFive".to_string(),
        "itemSix".to_string(),
        "itemSevenExtra".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::CamelCase),
        camel_expected
    );

    // SnakeCase
    let snake_expected = vec![
        "item_one".to_string(),
        "item_two".to_string(),
        "item_three".to_string(),
        "item_four".to_string(),
        "item_five".to_string(),
        "item_six".to_string(),
        "item_seven_extra".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::SnakeCase),
        snake_expected
    );

    // KebabCase
    let kebab_expected = vec![
        "item-one".to_string(),
        "item-two".to_string(),
        "item-three".to_string(),
        "item-four".to_string(),
        "item-five".to_string(),
        "item-six".to_string(),
        "item-seven-extra".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::KebabCase),
        kebab_expected
    );

    // PascalCase
    let pascal_expected = vec![
        "ItemOne".to_string(),
        "ItemTwo".to_string(),
        "ItemThree".to_string(),
        "ItemFour".to_string(),
        "ItemFive".to_string(),
        "ItemSix".to_string(),
        "ItemSevenExtra".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::PascalCase),
        pascal_expected
    );
}

#[test]
fn test_empty_and_whitespace_in_lists() {
    let input = vec![
        "hello_world".to_string(),
        "".to_string(),
        "foo-bar".to_string(),
        "   ".to_string(),
        "test case".to_string(),
    ];

    let camel_expected = vec![
        "helloWorld".to_string(),
        "".to_string(),
        "fooBar".to_string(),
        "".to_string(),
        "testCase".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::CamelCase),
        camel_expected
    );

    let snake_expected = vec![
        "hello_world".to_string(),
        "".to_string(),
        "foo_bar".to_string(),
        "".to_string(),
        "test_case".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::SnakeCase),
        snake_expected
    );
}

#[test]
fn test_mixed_formats_in_list() {
    let input = vec![
        "snake_case_string".to_string(),
        "kebab-case-string".to_string(),
        "PascalCaseString".to_string(),
        "camelCaseString".to_string(),
        "space separated string".to_string(),
        "MixedFormat_with-everything".to_string(),
    ];

    // To CamelCase
    let camel_expected = vec![
        "snakeCaseString".to_string(),
        "kebabCaseString".to_string(),
        "pascalCaseString".to_string(),
        "camelCaseString".to_string(),
        "spaceSeparatedString".to_string(),
        "mixedFormatWithEverything".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::CamelCase),
        camel_expected
    );

    // To SnakeCase
    let snake_expected = vec![
        "snake_case_string".to_string(),
        "kebab_case_string".to_string(),
        "pascal_case_string".to_string(),
        "camel_case_string".to_string(),
        "space_separated_string".to_string(),
        "mixed_format_with_everything".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::SnakeCase),
        snake_expected
    );

    // To KebabCase
    let kebab_expected = vec![
        "snake-case-string".to_string(),
        "kebab-case-string".to_string(),
        "pascal-case-string".to_string(),
        "camel-case-string".to_string(),
        "space-separated-string".to_string(),
        "mixed-format-with-everything".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::KebabCase),
        kebab_expected
    );

    // To PascalCase
    let pascal_expected = vec![
        "SnakeCaseString".to_string(),
        "KebabCaseString".to_string(),
        "PascalCaseString".to_string(),
        "CamelCaseString".to_string(),
        "SpaceSeparatedString".to_string(),
        "MixedFormatWithEverything".to_string(),
    ];
    assert_eq!(
        to_parse(input.clone(), StringMode::PascalCase),
        pascal_expected
    );
}

