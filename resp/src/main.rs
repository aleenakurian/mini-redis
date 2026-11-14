
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
    // have they arrived as byte string literal
    if input[cursor] != b'*' {
        return ParseResult::Invalid;
    }

    cursor += 1;
    //
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
        // extracting the length of the command
        let com_leng = match std::str::from_utf8(&input[cursor..crlf]) {
            Ok(s) => s,
            Err(_) => return ParseResult::Invalid,
        };
        //converting to usize
        let ucom_len = match com_leng.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return ParseResult::Invalid,
        };

        cursor = crlf + 2;

        // whether the complete data and trailing CRLF have arrived
        if cursor + ucom_len + 2 > input.len() {
            return ParseResult::Incomplete;
        }

        // Extract bytes
        let data = &input[cursor..cursor + ucom_len];

        // Verify the trailing CRLF
        if &input[cursor + ucom_len..cursor + ucom_len + 2] != b"\r\n" {
            return ParseResult::Invalid;
        }

        // Convert bytes into a String
        let command = match std::str::from_utf8(data) {
            Ok(s) => s.to_string(),
            Err(_) => return ParseResult::Invalid,
        };

        args.push(command);

        // Move cursor past the data and its CRLF
        cursor += ucom_len + 2;

    }
    return ParseResult::Complete(args, len);
    // ParseResult::Incomplete
}

fn main() {
    let input = b"*3\r\n$3\r\nSET\r\n$4\r\nname\r\n$5\r\nAlice\r\n";

    match parse(input) {
        ParseResult::Complete(args, consumed) => {
            println!("Args: {:?}", args);
            println!("Bytes consumed: {}", consumed);
        }
        ParseResult::Incomplete => println!("Incomplete input"),
        ParseResult::Invalid => println!("Invalid input"),
    }
}