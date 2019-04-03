pub fn first_word2(s: &String) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

