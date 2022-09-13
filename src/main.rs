use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();

    let emoj = '😂';

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
