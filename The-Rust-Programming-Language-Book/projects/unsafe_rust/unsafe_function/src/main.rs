fn main() {
    unsafe { dangerous() }
}

unsafe fn dangerous() {
    println!("unsafe method");
}
