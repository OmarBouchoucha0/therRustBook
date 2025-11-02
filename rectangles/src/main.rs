#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
fn main() {
    let rec1 = Rectangle {
        height: 40,
        width: 20,
    };
    println!("the area is {}", area(&rec1));
    println!("the height is {}", rec1.height);
    println!("the width is {}", rec1.width);
    println!("rect is {rec1:#?}");
}
