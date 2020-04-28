mod encoding {
    mod binary {
        use values::encoding::*;

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn encode_rfc4648_test_sample() {
            let mut output = String::with_capacity(8);
            encode_base64(&mut output, b"").unwrap();
            assert_eq!(output, "");
            output.clear();
            encode_base64(&mut output, b"f").unwrap();
            assert_eq!(output, "Zg==");
            output.clear();
            encode_base64(&mut output, b"fo").unwrap();
            assert_eq!(output, "Zm8=");
            output.clear();
            encode_base64(&mut output, b"foo").unwrap();
            assert_eq!(output, "Zm9v");
            output.clear();
            encode_base64(&mut output, b"foob").unwrap();
            assert_eq!(output, "Zm9vYg==");
            output.clear();
            encode_base64(&mut output, b"fooba").unwrap();
            assert_eq!(output, "Zm9vYmE=");
            output.clear();
            encode_base64(&mut output, b"foobar").unwrap();
            assert_eq!(output, "Zm9vYmFy");
        }

        #[test]
        fn encode_text() {
            let input =
                "Polyfon zwitschernd aßen Mäxchens Vögel Rüben, Joghurt und Quark".as_bytes();
            let mut output = String::with_capacity(input.len() + input.len() / 3);
            encode_base64(&mut output, input).unwrap();
            assert_eq!(output, "UG9seWZvbiB6d2l0c2NoZXJuZCBhw59lbiBNw6R4Y2hlbnMgVsO2Z2VsIFLDvGJlbiwgSm9naHVydCB1bmQgUXVhcms=");
        }

        #[test]
        fn decode_rfc4648_test_sample() {
            let mut output = Vec::with_capacity(8);
            decode_base64(&mut output, "");
            assert_eq!(output, b"");
            output.clear();
            decode_base64(&mut output, "Zg==");
            assert_eq!(output, b"f");
            output.clear();
            decode_base64(&mut output, "Zm8=");
            assert_eq!(output, b"fo");
            output.clear();
            decode_base64(&mut output, "Zm9v");
            assert_eq!(output, b"foo");
            output.clear();
            decode_base64(&mut output, "Zm9vYg==");
            assert_eq!(output, b"foob");
            output.clear();
            decode_base64(&mut output, "Zm9vYmE=");
            assert_eq!(output, b"fooba");
            output.clear();
            decode_base64(&mut output, "Zm9vYmFy");
            assert_eq!(output, b"foobar");
        }
        #[test]
        fn decode_text() {
            let input = "UG9seWZvbiB6d2l0c2NoZXJuZCBhw59lbiBNw6R4Y2hlbnMgVsO2Z2VsIFLDvGJlbiwgSm9naHVydCB1bmQgUXVhcms=";
            let mut output = Vec::with_capacity(input.len() - input.len() / 3);
            decode_base64(&mut output, input);
            assert_eq!(
                output,
                "Polyfon zwitschernd aßen Mäxchens Vögel Rüben, Joghurt und Quark".as_bytes()
            );
        }
    }

    mod text {
        use values::encoding::escape_text;

        #[test]
        fn escaped_chars() {
            let s = ",\r\n;:\\ \n \r\n";
            let expected = "\\,\n\\;:\\\\ \n \n";
            assert_eq!(expected, escape_text(s.into()));
        }

        #[test]
        fn no_escaped_chars() {
            let s = "This is a simple sentence.";
            let expected = s.clone();
            assert_eq!(expected, escape_text(s.into()));
        }

        // test run with default features enabled but should be correct regardless
        #[test]
        fn escape_property() {
            use components::Property;
            let s = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n";
            let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";
            let property = Property::new("COMMENT", escape_text(s.into()));
            assert_eq!(expected_value, property.value);
        }
    }
}

mod string {
    use values::string::*;

    // https://tools.ietf.org/html/rfc4648#section-10
    #[test]
    fn parse_valid_binary() {
        assert_eq!(Some(Binary::new(b"".as_ref())), "".parse().ok());
        assert_eq!(Some(Binary::new(b"f".as_ref())), "Zg==".parse().ok());
        assert_eq!(Some(Binary::new(b"fo".as_ref())), "Zm8=".parse().ok());
        assert_eq!(Some(Binary::new(b"foo".as_ref())), "Zm9v".parse().ok());
        assert_eq!(Some(Binary::new(b"foob".as_ref())), "Zm9vYg==".parse().ok());
        assert_eq!(
            Some(Binary::new(b"fooba".as_ref())),
            "Zm9vYmE=".parse().ok()
        );
        assert_eq!(
            Some(Binary::new(b"foobar".as_ref())),
            "Zm9vYmFy".parse().ok()
        );
    }

    #[test]
    fn parse_invalid_binary() {
        assert!("ABC".parse::<Binary>().is_err());
        assert!("Zö==".parse::<Binary>().is_err());
    }
}

mod time {
    use values::time::*;

    #[test]
    fn time_utc() {
        let expected = "173000Z";
        let time = Time::utc(17, 30, 0).unwrap();
        assert_eq!(time.to_string(), expected);
    }

    #[test]
    fn time_local() {
        let expected = "173000";
        let time = Time::local(17, 30, 0).unwrap();
        assert_eq!(time.to_string(), expected);
    }

    #[test]
    fn datetime_local() {
        let expected = "19970714T173000";
        let date = Date::new(1997, Month::July, 14).unwrap();
        let time = Time::local(17, 30, 0).unwrap();
        let datetime = DateTime::new(date, time);
        assert_eq!(datetime.to_string(), expected);
    }

    #[test]
    fn datetime_utc() {
        let expected = "19970714T173000Z";
        let date = Date::new(1997, Month::July, 14).unwrap();
        let time = Time::utc(17, 30, 0).unwrap();
        let datetime = DateTime::new(date, time);
        assert_eq!(datetime.to_string(), expected);
    }
}
