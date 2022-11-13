use std::io;

// fn main() {
//     println!("Hello, world!");

//     let mut guess = String::new();

//     let emoj = '😂';

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
