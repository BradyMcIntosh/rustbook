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
    println!("{s3}");

    let s4 = String::from("This is my ");
    let s5 = String::from("example!");
    let s6 = s4 + &s5;

    // println!("s4 is '{s4}'"); <-- s4 was consumed by + operator
    println!("s5 is '{s5}'");
    println!("s6 is '{s6}'");

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = format!("{s7}-{s8}-{s9}");

    println!("{s10}");
}
