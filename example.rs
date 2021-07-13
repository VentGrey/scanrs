extern crate scanrs;
use scanrs::*;

pub fn main() {
    println!("Please input a number:");
    let num: i64 = scann();
    println!("You entered \"{}\"", num);

    println!("Please input a list of numbers:");
    let vec: Vec<i64> = scanvec();
    println!("You entered \"{:?}\"", vec);

    println!("Please input a string:");
    let string = scanln();
    println!("You entered \"{}\"", string);
}
