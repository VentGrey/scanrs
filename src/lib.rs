use std::io;


/* Scan Integer Functions */
pub fn scand() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i64>().expect("Input error");
}

pub fn scani() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i64>().expect("Input error");
}

pub fn scand_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i32>().expect("Input error");
}

pub fn scani_32() -> i32 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i32>().expect("Input error");
}

/* Scan Float Functions */
pub fn scanfl() -> f64 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    return float.parse::<f64>().expect("Input Error");
}

pub fn scanfl_32() -> f32 {
    let mut float = String::new();
    io::stdin().read_line(&mut float).expect("IO ERROR");

    return float.parse::<f32>().expect("Input Error");
}

