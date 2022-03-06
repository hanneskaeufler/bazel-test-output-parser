extern crate nom;
use nom::{
    bytes::complete::{take_till1, take_while1},
    character::is_space,
    combinator::map,
    sequence::preceded,
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct TestLabel {
    name: String,
}

fn to_test_label(name: &[u8]) -> TestLabel {
    TestLabel {
        name: String::from_utf8(name.to_vec()).unwrap_or_default(),
    }
}

fn test_label_parser(input: &str) -> IResult<&[u8], TestLabel> {
    let start_of_label = take_while1(|c| c == b'/' || c == b':');
    let label = take_till1(is_space);
    let label_only = preceded(start_of_label, label);

    return map(label_only, to_test_label)(input.as_bytes());
}

pub fn parse(input: &str) -> Vec<TestLabel> {
    let parse_result = test_label_parser(input);

    if parse_result.is_ok() {
        return vec![parse_result.unwrap().1];
    }

    return vec![];
}

#[cfg(test)]
mod test {
    use super::{parse, TestLabel};

    fn label(name: &str) -> TestLabel {
        return TestLabel {
            name: name.to_string(),
        };
    }

    #[test]
    fn test_parse_with_single_toplevel_passing_test() {
        let buffer = "//:sometest      PASSING in 0.1";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_cached_passing_test() {
        let buffer = "//:sometest      (cached) PASSING in 0.1s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_failed_test() {
        let buffer = "//:sometest      FAILED in 0.1s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("sometest")])
    }

    #[test]
    fn test_parse_with_multiple_toplevel_tests() {
        let buffer = "//:sometest      FAILED in 0.1s\n//:othertest     PASSED in 0.5s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("sometest")])
    }

    #[test]
    fn test_parse_with_subpackage_test() {
        let buffer = "//some:sometest      FAILED in 0.1s\n";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("some:sometest")])
    }
}
