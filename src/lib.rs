use std::io;

/* Scan Integer Functions */
pub fn scand() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i64>().expect("Input error")
}

pub fn scani() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i64>().expect("Input error")
}

pub fn scand_8() -> i8 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i8>().expect("Input error")
}

pub fn scani_8() -> i8 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i8>().expect("Input error")
}

pub fn scand_16() -> i16 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i16>().expect("Input error")
}

pub fn scani_16() -> i16 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i16>().expect("Input error")
}

pub fn scand_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i32>().expect("Input error")
}

pub fn scani_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i32>().expect("Input error")
}

pub fn scand_128() -> i128 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i128>().expect("Input error")
}

pub fn scani_128() -> i128 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    int.trim().parse::<i128>().expect("Input error")
}

/* Scan Float Functions */
pub fn scanfl() -> f64 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    float.trim().parse::<f64>().expect("Input Error")
}

pub fn scanfl_32() -> f32 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    float.trim().parse::<f32>().expect("Input Error")
}

/* Scan a string */
pub fn scanln() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("IO ERROR");

    let trim: &[_] = &['\n', '\r'];
    string.trim_end_matches(trim).to_owned()
}

/* Scan Arrays (Whitespace separated) */

pub fn scanarr() -> Vec<i64> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
    .map(|x| x.parse::<i64>().expect("parse error"))
        .collect::<Vec<i64>>();

    vec
}

pub fn scanarr_8() -> Vec<i8> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
    .map(|x| x.parse::<i8>().expect("parse error"))
        .collect::<Vec<i8>>();

    vec
}

pub fn scanarr_16() -> Vec<i16> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
    .map(|x| x.parse::<i16>().expect("parse error"))
        .collect::<Vec<i16>>();

    vec
}

pub fn scanarr_32() -> Vec<i32> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
    .map(|x| x.parse::<i32>().expect("parse error"))
        .collect::<Vec<i32>>();

    vec
}

pub fn scanarr_128() -> Vec<i128> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
    .map(|x| x.parse::<i128>().expect("parse error"))
        .collect::<Vec<i128>>();

    vec
}

/* Scan an array of floating point numbers */


// Default function scans 64 bit arrays
pub fn scanfl_arr() -> Vec<f64> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
                   .map(|x| x.parse::<f64>().expect("parse error"))
                   .collect::<Vec<f64>>();
    vec
}

pub fn scanfl_arr_32() -> Vec<f32> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let vec = a_str.split_whitespace()
                   .map(|x| x.parse::<f32>().expect("parse error"))
                   .collect::<Vec<f32>>();
    vec
}
