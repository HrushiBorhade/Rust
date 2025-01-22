#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // All functions defined within an impl block are called associated functions because
    // they’re associated with the type named after the impl. We can define associated
    // functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with. We’ve already used one
    // function like this: the String::from function that’s defined on the String type.
    fn square(size:u32) -> Self {
        Self {
            width:size,
            height:size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 30,
    };

    let square1 = Rectangle::square(10);

    println!("rect1 is {rect1:#?}");
    println!("rect1 width: {}", rect1.width());
    println!("rect2 is {rect2:#?}");
    println!("square1 is {square1:#?}");
    println!("The area of rectangle : {}", rect1.area());
    println!("The area of rectangle : {}", rect2.area());
    println!("Can rect1 hold rect2 : {}", rect1.can_hold(&rect2));
}
