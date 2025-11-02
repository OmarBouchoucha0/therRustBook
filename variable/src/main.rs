fn main() {
    let heart_eyed_cat = '😻';
    let array: [u32; 3] = [1, 2, 3];
    println!("heart_eyed_cat : {heart_eyed_cat}");
    println!("array 1 : {}", array[2]);
    let s = String::from("Hello World!");
    let len = s.len();
    let cap = s.capacity();
    println!("string {s} length : {len} capacity = {cap}");
}
