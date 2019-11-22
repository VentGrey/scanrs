use std::io;


fn scand() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i64>().expect("Input error");
}

fn scani() -> i64 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("IO ERROR");

    return int.parse::<i64>().expect("Input error");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scand() {
        assert_eq!(2 + 2, 4);
    }
}
