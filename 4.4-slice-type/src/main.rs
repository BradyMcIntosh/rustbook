fn main() {
    let s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("slices: '{hello}' & '{world}'");

    let slice1: &str = &s[..5];
    let slice2: &str = &s[6..];
    let slice3: &str = &s[..];
    let slice4: &str = &s[..2];
}
