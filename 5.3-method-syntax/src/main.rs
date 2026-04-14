#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        ( self.width > other.width && self.height > other.height ) ||
        ( self.width > other.height && self.height > other.width )
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "This rectangle is {} pixels wide and {} pixels high. \nThe area of the rectangle is {} square pixels.",
        rect1.width(), rect1.height(), rect1.area()
    );

    println!(
        "Two more rectangles are {}x{} and {}x{}.",
        rect2.width(), rect2.height(), rect3.width(), rect3.height()
    );

    if rect1.can_hold(&rect2) {
        println!("Rectangle 1 can hold rectangle 2.");
    } else {
        println!("Rectangle 1 cannot hold rectangle 2.");
    }

    if rect1.can_hold(&rect3) {
        println!("Rectangle 1 can hold rectangle 3.");
    } else {
        println!("Rectangle 1 cannot hold rectangle 3.");
    }

    let sq1 = Rectangle::square(10);

    println!(
        "Here is a square: {}{}, area = {}.",
        sq1.width(), sq1.height(), sq1.area()
    );
}
