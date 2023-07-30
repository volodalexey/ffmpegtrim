use std::str::Split;

pub struct PartsFloat<'a> {
    #[allow(dead_code)]
    pub before_str: &'a str,
    pub before_float: f32,
    #[allow(dead_code)]
    pub after_str: &'a str,
    pub after_float: f32,
}

pub fn parse_float<'a>(str: &'a str, splitter: &'a str) -> PartsFloat<'a> {
    let mut parts: Split<&str> = str.split(splitter);

    let before_str = parts.next().expect("Can not detect before splitter").trim();
    let before_float: f32 = before_str.parse().unwrap_or(0.0);
    let after_str = parts.next().expect("Can not detect after splitter").trim();
    let after_float: f32 = after_str.parse().unwrap_or(0.0);

    return PartsFloat {
        before_str,
        before_float,
        after_str,
        after_float,
    };
}

#[cfg(test)]
mod tests {
    use super::parse_float;

    #[test]
    fn parse_float_for_nothing() {
        let parsed = parse_float("", "");
        assert_eq!(parsed.before_str, "");
        assert_eq!(parsed.after_str, "");
        assert_eq!(parsed.before_float, 0.0);
        assert_eq!(parsed.after_float, 0.0);
    }

    #[test]
    #[should_panic(expected = "Can not detect after splitter")]
    fn parse_float_panic_after() {
        parse_float("", "=");
    }

    #[test]
    fn parse_float_for_none_digit() {
        let parsed = parse_float("dd=gg", "=");
        assert_eq!(parsed.before_str, "dd");
        assert_eq!(parsed.after_str, "gg");
        assert_eq!(parsed.before_float, 0.0);
        assert_eq!(parsed.after_float, 0.0);
    }

    #[test]
    fn parse_float_for_positive_digit() {
        let parsed = parse_float("45.56dur5.2", "dur");
        assert_eq!(parsed.before_str, "45.56");
        assert_eq!(parsed.after_str, "5.2");
        assert_eq!(parsed.before_float, 45.56);
        assert_eq!(parsed.after_float, 5.2);
    }
}
