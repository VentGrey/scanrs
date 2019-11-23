# scanrs
A simple Rust Library to easily process std input.

## What is it?

This crate was a little inspired by this, so it won't have any dependencies
besides the standard library, anything extra should be done by hand or
cherrypicked to avoid bloating something this simple.

https://www.reddit.com/r/rust/comments/dy365h/only_one_wish_for_rust_2020/

scanrs is a small working rust crate that makes handling user input
easier.

Rust has a rather "weird" way to read std input, even though asking for user
input inside terminal applications is a little outdated.

But it's neccesary in some cases or in begginer-exercises to ask for a proper
input, sadly doing this in rust results in long lines of text and the end
program turns very verbose.

This library just attempts to make reading standard primitive types easier which
makes it faster, simple to understand and lightweight.

## How to use it?

This crate doesn't have much science behind it, to it's fairly simple to use,
you just call the function you wish.


### One way of reading user input in rust:

One simple way to read user input in rust is by playing a little with error
handling (or using a straight `unwrap` instead).

``` rust
let mut number = String::new();

io::stdin().read_line(&mut number).expect("IO Failure");

let number: f64 = match number.trim().parse() {
    Ok(num) => num,
    Err(_) => panic!("Not a number!"),
};
```

However this turns a little verbose when you need to read multiple variables, so
you can use this crate instead.

``` rust
use scanrs::scand;

fn main() {
    println!("Please input a number");
    let num = scand();
    println!("You entered {}", num);
}
```

## What kind of primitive types can it handle?

Well, I just developed this yesterday, so it can only read `f32`, `f64`, `i32` &
`i64` for now, but I'll keep developing this crate in the future to read
smaller data types, and a "manual" array index-by-index input.

Current functions available:
`scand` / `scani`: Reads an integer from stdin.
`scanfl`: Reads a float from stdin.

> Note: If you wish to use this with 32 bit data types you should use the `_32`
> postfix in the function name.

## TODO: 
