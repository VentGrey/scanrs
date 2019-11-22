#[macro_use()]
extern crate scanrs;

use scanrs::scand;

pub fn main() {
    println!("Please input a number");
    let num = scand();
    println!("You entered {}", num);

}
