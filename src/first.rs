use std::fs;

pub fn read(path: &str) -> String {
    return match fs::read_to_string(path) {
        Ok(r) => r,
        Err(_) => panic!("Cannot read content from static {}", path),
    };
}

pub fn parse(line: &str) -> (Option<(char)>, Option<(char)>) {
    let mut iter = line
        .chars()
        .enumerate()
        .filter(|e| e.1.is_numeric())
        .map(|e| e.1);

    return (iter.next(), iter.last());
}

pub fn convert(digits: (Option<(char)>, Option<(char)>)) -> u32 {
    match digits {
        (Some(c1), Some(c2)) => (c1.to_digit(10).unwrap() * 10) + (c2.to_digit(10).unwrap()),
        (Some(c1), None) => (c1.to_digit(10).unwrap() * 10) + (c1.to_digit(10).unwrap()),
        (None, Some(c2)) => (c2.to_digit(10).unwrap() * 10) + (c2.to_digit(10).unwrap()),
        (None, None) => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::first::{convert, parse};

    #[test]
    fn test_convert() {
        assert_eq!(convert((Some('1'), Some('2'))), 12);
        assert_eq!(convert((Some('7'), None)), 77);
        assert_eq!(convert((None, Some('7'))), 77);
        assert_eq!(convert((None, None)), 0);
    }

    #[test]
    fn test_read() {
        assert_eq!(parse("1abc2"), (Some('1'), Some('2')));
        assert_eq!(parse("pqr3stu8vwx"), (Some('3'), Some('8')));
        assert_eq!(parse("a1b2c3d4e5f"), (Some('1'), Some('5')));
        assert_eq!(parse("treb7uchet"), (Some('7'), None));
    }
}
