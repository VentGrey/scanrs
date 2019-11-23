use std::io;


/* Scan Integer Functions */
pub fn scand() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i64>().expect("Input error");
}

pub fn scani() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i64>().expect("Input error");
}

pub fn scand_8() -> i8 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i8>().expect("Input error");
}

pub fn scani_8() -> i8 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i8>().expect("Input error");
}

pub fn scand_16() -> i16 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i16>().expect("Input error");
}

pub fn scani_16() -> i16 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i16>().expect("Input error");
}

pub fn scand_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i32>().expect("Input error");
}

pub fn scani_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i32>().expect("Input error");
}

pub fn scand_128() -> i128 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i128>().expect("Input error");
}

pub fn scani_128() -> i128 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.trim().parse::<i128>().expect("Input error");
}

/* Scan Float Functions */
pub fn scanfl() -> f64 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    return float.trim().parse::<f64>().expect("Input Error");
}

pub fn scanfl_32() -> f32 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    return float.trim().parse::<f32>().expect("Input Error");
}

/* Scan a string */
pub fn scanln() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("IO ERROR");

    let trim: &[_] = &['\n', '\r'];
    return string.trim_end_matches(trim).to_owned();
}
