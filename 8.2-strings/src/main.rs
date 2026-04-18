fn main() {
    let mut hw = String::new();
    hw.push_str("Hello");
    hw.push_str(" ");
    hw.push_str("world");
    hw.push_str("!");

    println!("{hw}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");

    let mut s3 = String::from("hel");
    s3.push('p');
    println!("{s3}")
}
