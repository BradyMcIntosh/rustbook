use advanced_traits::*;

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(2001), Millimeters(1) + Meters(2));

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!(
        "A baby dog, as an animal, is called a {}",
        <Dog as Animal>::baby_name()
    );

    Point { x: 1, y: 3 }.outline_print();

    let w = VecWrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {w}");
}
