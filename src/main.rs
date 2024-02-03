// mod smart_pointer;

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

// use std::fs::File;

// fn main() {
//     // let greeting_file = File::open("hello.txt").unwrap();
//     // let greeting_file =
//     //     File::open("hello.txt").expect("hello.txt should be included in this project");

//     let s1 = String::from("hello");
//     println!("{}", s1.len());

//     let s2 = &s1;
//     println!("{}", s2.len());

//     // let len = calculate_length(&s1);
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
//     println!("{}", v.len());
// }

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// fn main() {
//     let x = 5;
//     // let y = &mut x;
//     let mut y = &x;
//     println!("{}", *y);

//     let z = 6;
//     y = &z;
//     println!("{}", *y);
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     println!("a next item = {:?}", a.tail());
// }

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:#?}", a);
//     println!("b after = {:#?}", b);
//     println!("c after = {:#?}", c);
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Main thread Got: {}", received);
//     }
// }

// #![allow(unused)]
// fn main() {
// use std::borrow::Cow;

// fn abs_all(input: &mut Cow<'_, [i32]>) {
//     for i in 0..input.len() {
//         let v = input[i];
//         if v < 0 {
//             // Clones into a vector if not already owned.
//             input.to_mut()[i] = -v;
//         }
//     }
// }

// // No clone occurs because `input` doesn't need to be mutated.
// let slice = [0, 1, 2];
// let mut input = Cow::from(&slice[..]);
// abs_all(&mut input);
// println!("{:?}", slice);
// println!("{:?}", input);

// // Clone occurs because `input` needs to be mutated.
// let slice = [-1, 0, 1];
// let mut input = Cow::from(&slice[..]);
// abs_all(&mut input);
// println!("{:?}", slice);
// println!("{:?}", input);

// // No clone occurs because `input` is already owned.
// let mut input = Cow::from(vec![-1, 0, 1]);
// abs_all(&mut input);
// println!("{:?}", slice);
// println!("{:?}", input);

// let mut setting_value = Some(5);
// let new_setting_value = Some(10);

// match (setting_value, new_setting_value) {
//     (Some(_), Some(_)) => {
//         println!("Can't overwrite an existing customized value");
//     }
//     _ => {
//         setting_value = new_setting_value;
//     }
// }

// println!("setting is {:?}", setting_value);

// enum Message {
//     Hello { id: i32 },
// }

// let msg = Message::Hello { id: 5 };

// match msg {
//     Message::Hello {
//         id: id_variable @ 3..=7,
//     } => println!("Found an id in range: {}", id_variable),
//     Message::Hello { id @ 10..=12 } => {
//         println!("Found an id {} in another range", id)
//     }
//     Message::Hello { id } => println!("Found some other id: {}", id),
// }

// use std::thread;

// let v = vec![1, 2, 3];

// let s: String = "Hello World!".to_string();
// let a = 1;

// fn a(a: u32) -> u23 {
//     a
// }
// let handle = thread::spawn(|| {
//     println!("Here's a vector: {:?}", v);
// });
// handle.join().unwrap();
// }

// fn main() {
//     // let mut x: i32 = 10;
//     // let ref_x = &mut x;
//     // *ref_x = 20;
//     // println!("x: {}", x);

//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     println!("a: {a:?}");

//     let s: &[i32] = &a[2..4];

//     println!("s: {s:?}");
// }

// use std::num::ParseIntError;
// use std::str::FromStr;

// #[derive(Debug, PartialEq)]
// struct Person {
//     name: String,
//     age: usize,
// }

// // We will use this error type for the `FromStr` implementation.
// #[derive(Debug, PartialEq)]
// enum ParsePersonError {
//     // Empty input string
//     Empty,
//     // Incorrect number of fields
//     BadLen,
//     // Empty name field
//     NoName,
//     // Wrapped error from parse::<usize>()
//     ParseInt(ParseIntError),
// }

// impl FromStr for Person {
//     type Err = ParsePersonError;
//     fn from_str(s: &str) -> Result<Person, Self::Err> {
//         if s.len() == 0 {
//             return Err(ParsePersonError::Empty);
//         }

//         let split_v: Vec<&str> = s.split(',').collect();
//         if split_v.len() != 2 {
//             return Err(ParsePersonError::BadLen);
//         }

//         if split_v[0].len() == 0 {
//             return Err(ParsePersonError::NoName);
//         }

//         match split_v[1].parse::<usize>() {
//             Ok(age) => Ok(Person {
//                 name: split_v[0].to_string(),
//                 age: age,
//             }),
//             Err(err) => Err(ParsePersonError::ParseInt(err)),
//         }
//     }
// }

// fn main() {
//     let p = "Mark,20".parse::<Person>().unwrap();
//     println!("{:?}", p);

//     let a = 2;
//     let b = &a;
//     let a = a + 1;
//     println!("{a} {}", *b);

//     println!("{:?}", second_word_to_upper("foo"));
// }

// fn second_word_to_upper(s: &str) -> Option<String> {
//     let mut it = s.split(' ');
//     let (Some(_), Some(item)) = (it.next(), it.next()) else {
//         return None;
//     };
//     Some(item.to_uppercase())
// }

// #[derive(Clone, Debug)]
// struct Point(i32, i32, String);

// fn main() {
//     let p1 = Point(3, 4, String::from("Hello"));
//     let p2 = p1.clone();
//     println!("p1: {p1:?}");
//     println!("p2: {p2:?}");
// }

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // This part is having a slight problem that is not current
        // The second item contains the IP and port of the new connection.
        // let (socket, _) = listener.accept().await.unwrap();
        // process(socket).await;

        // This part is doing well in current
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
