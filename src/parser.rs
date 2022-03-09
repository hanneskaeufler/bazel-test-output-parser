extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while, take_while1},
    character::is_space,
    combinator::map,
    sequence::{pair, preceded, terminated},
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct TestLabel {
    pub path: String,
    pub name: String,
}

fn to_test_label((path, name): (&[u8], &[u8])) -> TestLabel {
    TestLabel {
        path: String::from_utf8(path.to_vec()).unwrap_or_default(),
        name: String::from_utf8(name.to_vec()).unwrap_or_default(),
    }
}

fn test_label_parser(input: &[u8]) -> IResult<&[u8], TestLabel> {
    let start_of_label = tag("//");
    let label_target = take_till1(is_space);
    let label_path = take_while(|c| c != b':');
    let label_only = pair(
        preceded(start_of_label, label_path),
        preceded(tag(":"), label_target),
    );

    let test_result = pair(
        take_while1(is_space),
        alt((tag("PASSED"), tag("(cached) PASSED"), tag("FAILED"))),
    );

    map(terminated(label_only, test_result), to_test_label)(input)
}

pub fn parse(input: &str) -> Vec<TestLabel> {
    let mut test_labels = Vec::new();

    for line in input.lines() {
        let parse_result = test_label_parser(line.as_bytes());

        if let Ok(test_label) = parse_result {
            test_labels.push(test_label.1);
        }
    }

    test_labels
}

#[cfg(test)]
mod test {
    use super::{parse, TestLabel};

    fn pathless_label(name: &str) -> TestLabel {
        TestLabel {
            path: String::new(),
            name: name.to_string(),
        }
    }

    fn label(path: &str, name: &str) -> TestLabel {
        TestLabel {
            path: path.to_string(),
            name: name.to_string(),
        }
    }

    #[test]
    fn test_parse_with_single_toplevel_passing_test() {
        let buffer = "//:sometest      PASSED in 0.1";

        let tests = parse(buffer);

        assert_eq!(tests, vec![pathless_label("sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_cached_passing_test() {
        let buffer = "//:sometest      (cached) PASSED in 0.1s";

        let tests = parse(buffer);

        assert_eq!(tests, vec![pathless_label("sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_failed_test() {
        let buffer = "//:sometest      FAILED in 0.1s";

        let tests = parse(buffer);

        assert_eq!(tests, vec![pathless_label("sometest")])
    }

    #[test]
    fn test_parse_with_multiple_toplevel_tests() {
        let buffer = "//:sometest      FAILED in 0.1s\n//:othertest     PASSED in 0.5s";

        let tests = parse(buffer);

        assert_eq!(
            tests,
            vec![pathless_label("sometest"), pathless_label("othertest")]
        )
    }

    #[test]
    fn test_parse_with_subpackage_test() {
        let buffer = "//some:sometest      FAILED in 0.1s\n";

        let tests = parse(buffer);

        assert_eq!(tests, vec![label("some", "sometest")])
    }

    #[test]
    fn test_parse_ignores_skipped_tests() {
        let buffer = "//some:sometest      SKIPPED\n";

        let tests = parse(buffer);

        assert_eq!(tests, vec![])
    }

    #[test]
    fn test_parse_ignores_label_names_in_the_middle_of_other_lines() {
        let buffer = "some other output //some:sometest\n";

        let tests = parse(buffer);

        assert_eq!(tests, vec![])
    }
}
