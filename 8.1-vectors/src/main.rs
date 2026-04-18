fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("My vector: {v:?}");

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}."),
        None => println!("There is no fourth element.")
    }
}
