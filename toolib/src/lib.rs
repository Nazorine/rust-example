#[no_mangle]
pub extern fn add(x1: i32,x2: i32) -> i32 {
    let r: i32 = x1 + x2;
    return r
}
