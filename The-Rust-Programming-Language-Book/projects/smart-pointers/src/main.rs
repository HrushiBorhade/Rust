use crate::List::{Cons, Nil};
use std::ops::Deref;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    //associated type
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list : {:#?}", list);



    // Deref trait
    let x = 5;
    let y = &x;

    println!("y : {}", *y);

    let y  = MyBox::new(x);
    println!("y for MyBox: {}", *y);

    //Deref Coercion
    hello("Rust");
    hello(&MyBox("world"));
    
}

fn hello(name: &str) {
    println!("hello {name}");
}
