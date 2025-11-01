use std::io;

fn fibonacci(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
fn main() {
    let mut intput = String::new();
    println!("PUT NUMBER");
    io::stdin()
        .read_line(&mut intput)
        .expect("ERROR READING LINES");
    let number: i32 = intput.trim().parse().expect("ENTER A NUMBER");
    println!(
        "the fibonacci sequence of {} : {}",
        number,
        fibonacci(number),
    );
}
