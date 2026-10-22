use trait_objects::Draw;

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw the selectbox here
    }
}

fn main() {
    println!("Hello, world!");
}
