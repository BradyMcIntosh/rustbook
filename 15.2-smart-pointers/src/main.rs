use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y1 = &x;

    assert_eq!(5, x);
    assert_eq!(x, *y1);

    let y2 = Box::new(x);
    assert_eq!(x, *y2);

    let my_x = 5;
    let my_y = MyBox::new(x);

    assert_eq!(5, my_x);
    assert_eq!(my_x, *my_y);
}
