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

// use std::str::Chars;

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     let yellow_value_ref = scores.entry(String::from("Yellow")).or_insert(50);
//     *yellow_value_ref += 1;
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(*result, 100);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//     assert_eq!(*result, 'y');
// }

// fn print_slice<T>(v: &[T]) {
//     for x in v {
//         println!("{x}");
//     }
// }

// fn main() {
//     print_slice(&[1, 2, 3]);

//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     for i in &v {
//         println!("{}", i);
//     }
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl Point<i32> {
//     fn f(&self) -> &i32 {
//         &self.y
//     }
// }
// impl<T> Point<T> {
//     fn f(&self) -> &T {
//         &self.x
//     }
// }
// fn main() {
//     let p: Point<i32> = Point { x: 1, y: 2 };
//     println!("{}", p.f());
// }

// fn clonable<T: Clone>(t: T) -> impl Clone {
//     t
// }
// fn main() {
//     let s = String::from("hello");
//     let s2 = clonable(s);
//     println!("{}", s2.clone());
// }

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// impl<T: Display> ToString for T {
//     // --snip--
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

use std::fs::File;

fn main() {
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    let s1 = String::from("hello");
    println!("{}", s1.len());

    let s2 = &s1;
    println!("{}", s2.len());

    // let len = calculate_length(&s1);
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("{}", v.len());
}
