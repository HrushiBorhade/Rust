enum Message {
    Quit, 
    Move {x:u32, y:u32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
