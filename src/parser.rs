#[derive(PartialEq, Debug)]
pub struct TestLabel {
    name: String,
}

pub fn parse(_output: &str) -> Vec<TestLabel> {
    return vec![];
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn test_parse_with_single_toplevel_passing_test() {
        let buffer = "//:sometest      PASSING in 0.1";

        let tests = parse(&buffer);

        assert_ne!(tests, vec![])
    }

    #[test]
    fn test_parse_with_single_toplevel_cached_passing_test() {
        let buffer = "//:sometest      (cached) PASSING in 0.1s";

        let tests = parse(&buffer);

        assert_ne!(tests, vec![])
    }

    #[test]
    fn test_parse_with_single_toplevel_failed_test() {
        let buffer = "//:sometest      FAILED in 0.1s";

        let tests = parse(&buffer);

        assert_ne!(tests, vec![])
    }
}
