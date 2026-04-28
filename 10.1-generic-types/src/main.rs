fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let p_integer = Point { x: 5, y: 10 };
    let p_float = Point { x: 1.1, y: 4.4 };

    println!("Int point: {p_integer:?}, float point: {p_float:?}!");

    let p_mix = Point { x: 0, y: 3.14159 };
    println!("Mix: {p_mix:?}!");
    println!("Getting elements: '{}', '{}'", p_mix.x(), p_mix.y());

    let p_boggle = p_integer.mixup(p_float);
    println!("Mixup function: {p_boggle:?}!");
}
