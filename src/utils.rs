pub fn input(msg: &str, expect_length: bool, remove_newline_char: bool) -> String {
    println!("{}", msg);
    let mut buf = String::new();
    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        if !expect_length {
            break buf;
        };
        if buf.len() > 0 {
            break if remove_newline_char {
                String::from(
                    buf.as_str()
                        .strip_suffix("\r\n")
                        .or(buf.strip_suffix("\n"))
                        .unwrap_or(buf.as_str()),
                )
            } else {
                buf
            };
        } else {
            println!("Please type something...");
        }
    }
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}
