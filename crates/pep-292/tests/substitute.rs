use std::collections::HashMap;

use pep_292::Template;
use pep_292::TemplateError;
use rstest::*;
use speculoos::assert_that;

#[rstest]
#[case("hello", HashMap::new(), Ok("hello".into()))]
#[case("I $feeling string substitution",HashMap::from([("feeling", "love")]) , Ok("I love string substitution".into()))]
#[trace]
fn test_substitute(
    #[case] input: &str,
    #[case] map: HashMap<&str, &str>,
    #[case] expected: Result<String, TemplateError>,
) {
    let res = input.substitute(&map);
    assert_that!(res).is_equal_to(expected)
}
