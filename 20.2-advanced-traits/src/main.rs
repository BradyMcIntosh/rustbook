use advanced_traits::*;

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(2001), Millimeters(1) + Meters(2));

    let person = Human;
    // person.fly(); <-- this doesn't actually work
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Human::fly(&person); <-- this also doesn't work
}
