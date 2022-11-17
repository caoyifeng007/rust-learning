use std::io;

// fn main() {
//     println!("Hello, world!");

//     let mut guess = String::new();

//     let emoj = '😂';

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point
//     println!("{} and {}", r1, r2);

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

// fn main() {
//     let mut s = String::from("Hello world");
//     let s2 = &mut s;
//     let s3 = s2;
//     println!("{}", s2); // Not valid because s2 is moved
// }

fn decr_twice_v1(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => {
            println!("{}", n2);
            Some(n2 - 2)
        }
    }
}

fn decr_twice_v2(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}
fn main() {
    let out = decr_twice_v1(2);
}
