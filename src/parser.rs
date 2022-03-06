#[derive(PartialEq, Debug)]
pub struct TestLabel {
    name: String,
}

pub fn parse(_output: &str) -> Vec<TestLabel> {
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

        assert_eq!(tests, vec![label(":sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_cached_passing_test() {
        let buffer = "//:sometest      (cached) PASSING in 0.1s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label(":sometest")])
    }

    #[test]
    fn test_parse_with_single_toplevel_failed_test() {
        let buffer = "//:sometest      FAILED in 0.1s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label(":sometest")])
    }

    #[test]
    fn test_parse_with_multiple_toplevel_tests() {
        let buffer = "//:sometest      FAILED in 0.1s\n//:othertest     PASSED in 0.5s";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label(":sometest")])
    }

    #[test]
    fn test_parse_with_subpackage_test() {
        let buffer = "//some:sometest      FAILED in 0.1s\n";

        let tests = parse(&buffer);

        assert_eq!(tests, vec![label("some:sometest")])
    }
}
