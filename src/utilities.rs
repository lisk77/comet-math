pub fn fac(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * fac(n-1)
    }
}
