fn main() {
    let x = 5;
    let y1 = &x;

    assert_eq!(5, x);
    assert_eq!(x, *y1);

    let y2 = Box::new(x);
    assert_eq!(x, *y2);
}
