mod contentline {
    use core::contentline::*;

    #[test]
    fn no_linebreak() {
        let content = "No line break today.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();

        assert_eq!(line, content);
    }

    #[test]
    fn over_limit() {
        let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";

        assert_eq!(line, expected);
    }

    #[test]
    fn multibytes() {
        let content = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";

        assert_eq!(line, expected);
    }

    #[test]
    fn multi_lines() {
        let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown\r\n  fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";

        assert_eq!(line, expected);
    }
}
