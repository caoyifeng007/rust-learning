use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Vehicle;

fn main() {
    Pancakes::hello_macro();

    Vehicle::hello_macro();
}
