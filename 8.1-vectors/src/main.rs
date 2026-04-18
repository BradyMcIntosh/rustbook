#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

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

    for i in &v {
        println!("Number: {i}!")
    }

    let mut w = vec![111, 64, 2];
    println!("New vector: {w:?}");

    for i in &mut w {
        *i += 50;
    }
    println!("After modification: {w:?}");

    let row = vec![
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Float(0.7),
        SpreadsheetCell::Text(String::from("example")),
    ];

    println!("My spreadsheet row: {row:?}");
}
