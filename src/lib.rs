use std::io;
use std::str::FromStr;

/* Scan a number */

pub fn scann<T: FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("IO error");

    number.trim().parse::<T>().expect("Input error")
}

/* Scan a list of elements (Whitespace separated) */

pub fn scanvec<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: std::fmt::Debug {
    let mut vec = String::new();
    
    io::stdin().read_line(&mut vec).expect("IO error");

    let vec = vec
        .split_whitespace()
        .map(|x| x.parse::<T>().expect("Parse error"))
        .collect::<Vec<T>>();

    vec
}

/* Scan a string */

pub fn scanln() -> String {
    let mut string = String::new();
    
    io::stdin().read_line(&mut string).expect("IO ERROR");

    let trim: &[_] = &['\n', '\r'];
    string.trim_end_matches(trim).to_owned()
}
