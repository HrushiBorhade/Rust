fn main() {
    let mut x = 5;

    let r1 = &mut x as *mut i32;
    let r2 = &x as *const i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}
