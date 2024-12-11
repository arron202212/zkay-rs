pub fn zk_div(a: i32, b: i32) -> i32 {
    if (a < 0) ^ (b < 0) {
        -(-a / b)
    } else {
        a / b
    }
}
pub fn zk_mod(a: i32, b: i32) -> i32 {
    let sign = if a < 0 { -1 } else { 1 };
    let abs_res = a.abs() % b.abs();
    sign * abs_res
}
