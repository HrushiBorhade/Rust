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

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width:30,
        height:50,
    };
    
    let rect2 = Rectangle {
        width:10,
        height:30,
    };

    println!("rect1 is {rect1:#?}");
    println!("rect1 width: {}" , rect1.width());
    println!("rect2 is {rect2:#?}");
    println!("The area of rectangle : {}", rect1.area());
    println!("The area of rectangle : {}", rect2.area());
    println!("Can rect1 hold rect2 : {}", rect1.can_hold(&rect2));

}

