#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.area() >= rec.area()
    }
}
fn main() {
    let rec1 = Rectangle::new(40, 60);
    let rec2 = Rectangle::new(40, 60);
    if rec1.can_hold(&rec2) {
        println!("rec 1 can hold rec 2");
    } else {
        println!("rec 2 can hold rec 1");
    }
}
