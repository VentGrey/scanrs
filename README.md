# scanrs

A dead-simple Rust Library to easily process std input. No dependencies, no strange functionalities, not even useful enough. You can always copy-paste the code inside `src/lib.rs`.

## What is it?

This crate was a little inspired by this, so it won't have any dependencies
besides the standard library, anything extra should be done by hand or
cherrypicked to avoid bloating something this simple.

https://www.reddit.com/r/rust/comments/dy365h/only_one_wish_for_rust_2020/

> 2023 update: I've realized the terrible mistake I've made by publishing this. Adding up to the dependency-hell problem because devs are independent-no-more. I'll keep maintaining this "feature complete" crate. However, I plead to you, please, don't be an npm user and get a library for dead-simple things :) checkout [rust easy snippets for this](https://github.com/VentGrey/rust-easy-snippets)

scanrs is a small working rust crate that makes handling user input
easier.

Rust has a rather ~~"weird"~~ verbose way to read std input, even though asking for user
input inside terminal applications is a little outdated. If you cannot rely on the command-line arguments for reliable input, this crate can prove itself useful.

The other case-scenario in my head is to use this to aid new rustaceans to ask for a proper
input, sadly doing this in rust results in long lines of text and the end program turns very verbose (not really).

This library just attempts to make reading standard primitive types (and two more cases) easier which
makes it faster, simple to understand and lightweight.

## How to use it?

This crate doesn't have much science behind it, to it's fairly simple to use,
you just assign a variable and set it's value to the result of the function you wish.


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

or

``` rust
let mut number = String::new();
io::stdin().read_line(&mut number).expect("IO Failure");

match input.trim().parse() {
    Ok(number) => number,
    Err(_) => panic!("Invalid input. Please enter a valid number."),
}

```

However this turns a little verbose when you need to read multiple variables, so
you can use this crate instead.

``` rust
use scanrs::scann;

fn main() {
    println!("Please input a number");
    let num = scann();
    println!("You entered {}", num);
}
```

## What kind of primitive types can it handle?

Current functions available:

* `scann`: Reads a number from stdin.
* `scanvec`: Scans a whitespace separated list of elements
* `scanln`: Scans a String

## TODO: 
- [ ] ~~Allow users to customize input message.~~ - Dimmed as bloatware
- [ ] Allow users to customize the function error message without introducing a breaking change.
- [ ] ~~Make this crate colored to send fancy errors.~~ - Dimmed as bloatware
