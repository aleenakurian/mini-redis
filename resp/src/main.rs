pub enum ParseResult {
    Complete(Vec<String>, usize),
    Incomplete,
    Invalid,
}

pub fn parse(input: &[u8]) -> ParseResult {
    let mut cursor: usize = 0;

    if input.is_empty() {
        return ParseResult::Incomplete;
    }

    if input[cursor] != b'*' {
        return ParseResult::Invalid;
    }

    cursor += 1;

    let crlf = match input[cursor..].windows(2).position(|w| w == b"\r\n") {
        Some(x) => cursor + x,
        None => return ParseResult::Incomplete,
    };

    let leng = match std::str::from_utf8(&input[cursor..crlf]) {
        Ok(s) => s,
        Err(_) => return ParseResult::Invalid,
    };

    let len = match leng.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return ParseResult::Invalid,
    };

    cursor = crlf + 2;

    let mut args = Vec::new();

    for _ in 0..len {
        if cursor >= input.len() {
            return ParseResult::Incomplete;
        }

        if input[cursor] != b'$' {
            return ParseResult::Invalid;
        }

        cursor += 1;

        let crlf = match input[cursor..].windows(2).position(|w| w == b"\r\n") {
            Some(x) => cursor + x,
            None => return ParseResult::Incomplete,
        };

        let com_leng = match std::str::from_utf8(&input[cursor..crlf]) {
            Ok(s) => s,
            Err(_) => return ParseResult::Invalid,
        };

        let ucom_len = match com_leng.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return ParseResult::Invalid,
        };

        cursor = crlf + 2;

    }

    ParseResult::Incomplete
}