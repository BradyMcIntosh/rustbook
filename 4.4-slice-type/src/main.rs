fn main() {
    let s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("slices: '{hello}' & '{world}'");

    let _slice1: &str = &s[..5];
    let _slice2: &str = &s[6..];
    let _slice3: &str = &s[..];
    let _slice4: &str = &s[..2];

    let firstword = first_word(&s);
    println!("first word: '{firstword}'")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
