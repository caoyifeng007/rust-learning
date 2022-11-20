// use std::io;

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

// fn decr_twice_v1(n: u32) -> Option<u32> {
//     match n {
//         0 => None,
//         1 => None,
//         n2 => {
//             println!("{}", n2);
//             Some(n2 - 2)
//         }
//     }
// }

// fn decr_twice_v2(n: u32) -> Option<u32> {
//     if n == 0 {
//         None
//     } else if n == 1 {
//         None
//     } else {
//         Some(n - 2)
//     }
// }
// fn main() {
//     let out = decr_twice_v1(2);
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[2];

//     v.push(6);

//     println!("The first element is: {}", first);
// }

// fn main() {
//     let mut s = String::from("foo");
//     s.push_str("bar");
// }

// fn main() {
//     let mut v = Vec::new();
//     let s = String::from("Hello ");
//     v.push(s);
//     v[0].push_str("world");
//     println!("original: {}", s);
//     println!("new: {}", v[0]);
// }
//     println!("new: {}", v[0]);

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = s1 + "-" + &s2 + "-" + &s3;

//     println!("{}", s1)
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{}-{}-{}", s1, s2, s3);

//     println!("{}", s1)
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);

//     println!("{}", score);
// }

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let yellow_value_ref = scores.entry(String::from("Yellow")).or_insert(50);
    *yellow_value_ref += 1;
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
