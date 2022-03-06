pub fn parse(_output: &str) {}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn test_parse_with_single_toplevel_passing_test() {
        let buffer = "//:sometest      PASSING in 0.1";
        parse(&buffer);

        assert!(false)
    }

    #[test]
    fn test_parse_with_single_toplevel_cached_passing_test() {
        let buffer = "//:sometest      (cached) PASSING in 0.1s";
        parse(&buffer);

        assert!(false)
    }

    #[test]
    fn test_parse_with_single_toplevel_failed_test() {
        let buffer = "//:sometest      FAILED in 0.1s";
        parse(&buffer);

        assert!(false)
    }
}
