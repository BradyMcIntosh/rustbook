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

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
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
        "Two more rectangles are 2: {}x{} and 3: {}x{}.",
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

    let rect4 = Rectangle {
        width: 1,
        height: 100,
    };

    println!(
        "Rect4 = {}x{} | {}",
        rect4.width(), rect4.height(), rect4.area()
    );

    let rect5 = rect4.max(rect3);

    println!(
        "Rect 5 is the max of rect3 and rect4.\nDimensions are {}x{} | {}",
        rect5.width(), rect5.height(), rect5.area()
    )

}
